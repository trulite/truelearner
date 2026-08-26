use dioxus::desktop::wry::http::{
    header::{ACCEPT_RANGES, ALLOW, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, RANGE},
    HeaderValue, Method, Request, Response, StatusCode,
};
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use std::borrow::Cow;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Component, Path, PathBuf};

pub const SCHEME: &str = "academy-media";
const MAX_RANGE_BYTES: u64 = 1024 * 1024;

#[derive(Clone, Debug)]
pub struct MediaRoot {
    path: PathBuf,
}

impl MediaRoot {
    pub fn new(path: PathBuf) -> Self {
        let path = fs::canonicalize(&path).unwrap_or_else(|_| absolute_path(path));
        Self { path }
    }

    pub fn respond(&self, request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
        match self.read(&request) {
            Ok(response) => response.map(Cow::Owned),
            Err(error) => error.response(),
        }
    }

    fn read(&self, request: &Request<Vec<u8>>) -> Result<Response<Vec<u8>>, MediaError> {
        if request.method() != Method::GET && request.method() != Method::HEAD {
            return Err(MediaError::MethodNotAllowed);
        }

        let path = self.resolve(request.uri().path())?;
        let mut file = File::open(&path).map_err(|_| MediaError::Io)?;
        let length = file.metadata().map_err(|_| MediaError::Io)?.len();
        let byte_range = requested_range(request.headers().get(RANGE), length)?;
        let (status, start, end) = match byte_range {
            Some(byte_range) => (
                StatusCode::PARTIAL_CONTENT,
                byte_range.start,
                byte_range.end,
            ),
            None => (StatusCode::OK, 0, length.saturating_sub(1)),
        };
        let response_length = if length == 0 { 0 } else { end + 1 - start };

        let mut body = Vec::new();
        if request.method() == Method::GET && response_length > 0 {
            body.reserve(response_length as usize);
            file.seek(SeekFrom::Start(start))
                .map_err(|_| MediaError::Io)?;
            file.take(response_length)
                .read_to_end(&mut body)
                .map_err(|_| MediaError::Io)?;
        }

        let mut response = Response::builder()
            .status(status)
            .header(CONTENT_TYPE, content_type(&path))
            .header(ACCEPT_RANGES, "bytes")
            .header(CONTENT_LENGTH, response_length);
        if byte_range.is_some() {
            response = response.header(CONTENT_RANGE, format!("bytes {start}-{end}/{length}"));
        }
        response.body(body).map_err(|_| MediaError::Http)
    }

    fn resolve(&self, encoded_path: &str) -> Result<PathBuf, MediaError> {
        let encoded_path = encoded_path.strip_prefix('/').unwrap_or(encoded_path);
        let decoded = percent_decode_str(encoded_path)
            .decode_utf8()
            .map_err(|_| MediaError::InvalidPath)?;
        let relative = Path::new(decoded.as_ref());
        if relative.as_os_str().is_empty()
            || !relative
                .components()
                .all(|component| matches!(component, Component::Normal(_)))
        {
            return Err(MediaError::InvalidPath);
        }

        let path = fs::canonicalize(self.path.join(relative)).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                MediaError::NotFound
            } else {
                let _ = error;
                MediaError::Io
            }
        })?;
        if !path.starts_with(&self.path) || !path.is_file() {
            return Err(MediaError::NotFound);
        }
        Ok(path)
    }
}

pub fn uri(relative_path: &str) -> String {
    let portable_path = relative_path.replace('\\', "/");
    let encoded = utf8_percent_encode(&portable_path, NON_ALPHANUMERIC);
    format!("{SCHEME}://localhost/{encoded}")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ByteRange {
    start: u64,
    end: u64,
}

fn requested_range(
    header: Option<&HeaderValue>,
    length: u64,
) -> Result<Option<ByteRange>, MediaError> {
    let Some(header) = header else {
        return Ok(None);
    };
    if length == 0 {
        return Err(MediaError::RangeNotSatisfiable(length));
    }

    let value = header
        .to_str()
        .map_err(|_| MediaError::RangeNotSatisfiable(length))?;
    let value = value
        .strip_prefix("bytes=")
        .ok_or(MediaError::RangeNotSatisfiable(length))?;
    if value.contains(',') {
        return Err(MediaError::RangeNotSatisfiable(length));
    }
    let (start, end) = value
        .split_once('-')
        .ok_or(MediaError::RangeNotSatisfiable(length))?;

    let (start, requested_end) = if start.is_empty() {
        let suffix = end
            .parse::<u64>()
            .map_err(|_| MediaError::RangeNotSatisfiable(length))?;
        if suffix == 0 {
            return Err(MediaError::RangeNotSatisfiable(length));
        }
        (length.saturating_sub(suffix), length - 1)
    } else {
        let start = start
            .parse::<u64>()
            .map_err(|_| MediaError::RangeNotSatisfiable(length))?;
        if start >= length {
            return Err(MediaError::RangeNotSatisfiable(length));
        }
        let requested_end = if end.is_empty() {
            length - 1
        } else {
            end.parse::<u64>()
                .map_err(|_| MediaError::RangeNotSatisfiable(length))?
                .min(length - 1)
        };
        if requested_end < start {
            return Err(MediaError::RangeNotSatisfiable(length));
        }
        (start, requested_end)
    };
    let end = requested_end.min(start.saturating_add(MAX_RANGE_BYTES - 1));

    Ok(Some(ByteRange { start, end }))
}

fn content_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("mp4") => "video/mp4",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

fn absolute_path(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .map(|current| current.join(&path))
            .unwrap_or(path)
    }
}

#[derive(Debug)]
enum MediaError {
    InvalidPath,
    NotFound,
    MethodNotAllowed,
    RangeNotSatisfiable(u64),
    Io,
    Http,
}

impl MediaError {
    fn response(self) -> Response<Cow<'static, [u8]>> {
        let range_length = match &self {
            Self::RangeNotSatisfiable(length) => Some(*length),
            _ => None,
        };
        let method_not_allowed = matches!(self, Self::MethodNotAllowed);
        let (status, message) = match self {
            Self::InvalidPath | Self::NotFound => (StatusCode::NOT_FOUND, "not found"),
            Self::MethodNotAllowed => (StatusCode::METHOD_NOT_ALLOWED, "method not allowed"),
            Self::RangeNotSatisfiable(_) => {
                (StatusCode::RANGE_NOT_SATISFIABLE, "range not satisfiable")
            }
            Self::Io | Self::Http => (StatusCode::INTERNAL_SERVER_ERROR, "media read failed"),
        };
        let mut response = Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "text/plain; charset=utf-8");
        if method_not_allowed {
            response = response.header(ALLOW, "GET, HEAD");
        }
        if let Some(length) = range_length {
            response = response.header(CONTENT_RANGE, format!("bytes */{length}"));
        }
        response
            .body(Cow::Borrowed(message.as_bytes()))
            .expect("static media error response is valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock is after the Unix epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "academy-media-{}-{nonce}-{}",
                std::process::id(),
                NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).expect("create isolated media test directory");
            Self(path)
        }

        fn write(&self, relative: &str, bytes: &[u8]) {
            let path = self.0.join(relative);
            fs::create_dir_all(path.parent().unwrap()).expect("create media directory");
            fs::write(path, bytes).expect("write media fixture");
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn complete_request_preserves_file_bytes() {
        let directory = TestDirectory::new();
        directory.write("episodes/one/poster.png", b"poster bytes");
        let root = MediaRoot::new(directory.0.clone());

        let response = root.respond(request(Method::GET, &uri("episodes/one/poster.png"), None));

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CONTENT_TYPE], "image/png");
        assert_eq!(response.body().as_ref(), b"poster bytes");
    }

    #[test]
    fn range_request_preserves_the_requested_slice() {
        let directory = TestDirectory::new();
        directory.write("episodes/one/episode.mp4", b"0123456789");
        let root = MediaRoot::new(directory.0.clone());

        let response = root.respond(request(
            Method::GET,
            &uri("episodes/one/episode.mp4"),
            Some("bytes=2-5"),
        ));

        assert_eq!(response.status(), StatusCode::PARTIAL_CONTENT);
        assert_eq!(response.headers()[CONTENT_RANGE], "bytes 2-5/10");
        assert_eq!(response.headers()[CONTENT_LENGTH], "4");
        assert_eq!(response.body().as_ref(), b"2345");
    }

    #[test]
    fn head_reports_length_without_reading_a_body() {
        let directory = TestDirectory::new();
        directory.write("record.json", b"evidence");
        let root = MediaRoot::new(directory.0.clone());

        let response = root.respond(request(Method::HEAD, &uri("record.json"), None));

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CONTENT_LENGTH], "8");
        assert!(response.body().is_empty());
    }

    #[test]
    fn encoded_parent_traversal_is_rejected() {
        let directory = TestDirectory::new();
        directory.write("inside.json", b"inside");
        let root = MediaRoot::new(directory.0.clone());

        let response = root.respond(request(
            Method::GET,
            "academy-media://localhost/%2E%2E/outside.json",
            None,
        ));

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(response.body().as_ref(), b"not found");
    }

    #[test]
    fn unsatisfiable_range_returns_no_file_bytes() {
        let directory = TestDirectory::new();
        directory.write("episode.mp4", b"0123456789");
        let root = MediaRoot::new(directory.0.clone());

        let response = root.respond(request(
            Method::GET,
            &uri("episode.mp4"),
            Some("bytes=20-30"),
        ));

        assert_eq!(response.status(), StatusCode::RANGE_NOT_SATISFIABLE);
        assert_eq!(response.headers()[CONTENT_RANGE], "bytes */10");
        assert_eq!(response.body().as_ref(), b"range not satisfiable");
    }

    fn request(method: Method, uri: &str, range: Option<&str>) -> Request<Vec<u8>> {
        let mut request = Request::builder().method(method).uri(uri);
        if let Some(range) = range {
            request = request.header(RANGE, range);
        }
        request.body(Vec::new()).expect("valid media request")
    }
}
