//! Durable Academy evidence storage.
//!
//! This crate is intentionally outside `truelearner-core`: object storage is
//! Academy infrastructure and has no effect on organism physics.

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{primitives::ByteStream, Client};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{env, error::Error, fmt};

pub const DEFAULT_REGION: &str = "ap-south-1";
pub const DEFAULT_PREFIX: &str = "academy/v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlobKind {
    Body,
    Checkpoint,
    PhysicalInput,
    Surface,
    Episode,
    Thumbnail,
    Manifest,
}

impl BlobKind {
    pub const fn path(self) -> &'static str {
        match self {
            Self::Body => "bodies",
            Self::Checkpoint => "checkpoints",
            Self::PhysicalInput => "physical-inputs",
            Self::Surface => "surfaces",
            Self::Episode => "episodes",
            Self::Thumbnail => "thumbnails",
            Self::Manifest => "manifests",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContentHash(String);

impl ContentHash {
    pub fn of(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);
        Self(hex::encode(digest))
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, StorageError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(StorageError::InvalidContentHash(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlobRef {
    pub kind: BlobKind,
    pub sha256: ContentHash,
    pub bytes: u64,
    pub key: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S3StoreConfig {
    pub bucket: String,
    pub region: String,
    pub prefix: String,
}

impl S3StoreConfig {
    pub fn from_env() -> Result<Self, StorageError> {
        let bucket = env::var("ACADEMY_S3_BUCKET")
            .map_err(|_| StorageError::MissingEnvironment("ACADEMY_S3_BUCKET"))?;
        let region = env::var("ACADEMY_S3_REGION").unwrap_or_else(|_| DEFAULT_REGION.to_owned());
        let prefix = env::var("ACADEMY_S3_PREFIX").unwrap_or_else(|_| DEFAULT_PREFIX.to_owned());
        Self::new(bucket, region, prefix)
    }

    pub fn new(
        bucket: impl Into<String>,
        region: impl Into<String>,
        prefix: impl Into<String>,
    ) -> Result<Self, StorageError> {
        let bucket = bucket.into();
        let region = region.into();
        let prefix = prefix.into().trim_matches('/').to_owned();
        if bucket.trim().is_empty() {
            return Err(StorageError::InvalidConfiguration(
                "bucket must not be empty",
            ));
        }
        if region.trim().is_empty() {
            return Err(StorageError::InvalidConfiguration(
                "region must not be empty",
            ));
        }
        if prefix.is_empty() {
            return Err(StorageError::InvalidConfiguration(
                "prefix must not be empty",
            ));
        }
        Ok(Self {
            bucket,
            region,
            prefix,
        })
    }

    pub fn object_key(&self, kind: BlobKind, hash: &ContentHash) -> String {
        let digest = hash.as_str();
        format!(
            "{}/{}/{}/{}/{}",
            self.prefix,
            kind.path(),
            &digest[..2],
            &digest[2..4],
            digest
        )
    }
}

#[derive(Clone)]
pub struct S3AcademyStore {
    client: Client,
    config: S3StoreConfig,
}

impl S3AcademyStore {
    pub async fn load(config: S3StoreConfig) -> Self {
        let sdk = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(config.region.clone()))
            .load()
            .await;
        Self {
            client: Client::new(&sdk),
            config,
        }
    }

    pub fn config(&self) -> &S3StoreConfig {
        &self.config
    }

    /// Store immutable bytes under their SHA-256 identity.
    ///
    /// Repeated writes are read-before-write deduplicated. A concurrent writer
    /// may create another S3 version, but it can only place the same bytes at
    /// the same content-derived key.
    pub async fn put_blob(
        &self,
        kind: BlobKind,
        bytes: impl Into<Vec<u8>>,
        content_type: &str,
    ) -> Result<BlobRef, StorageError> {
        let bytes = bytes.into();
        let sha256 = ContentHash::of(&bytes);
        let key = self.config.object_key(kind, &sha256);
        let reference = BlobRef {
            kind,
            sha256: sha256.clone(),
            bytes: bytes.len() as u64,
            key: key.clone(),
        };

        match self
            .client
            .head_object()
            .bucket(&self.config.bucket)
            .key(&key)
            .send()
            .await
        {
            Ok(existing) => {
                let existing_hash = existing.metadata().and_then(|map| map.get("sha256"));
                if existing_hash.is_some_and(|hash| hash == sha256.as_str())
                    && existing.content_length().unwrap_or_default() == bytes.len() as i64
                {
                    return Ok(reference);
                }
                return Err(StorageError::ContentAddressCollision(key));
            }
            Err(error)
                if error
                    .as_service_error()
                    .is_some_and(|service| service.is_not_found()) => {}
            Err(error) => return Err(StorageError::S3(error.to_string())),
        }

        self.client
            .put_object()
            .bucket(&self.config.bucket)
            .key(&key)
            .content_type(content_type)
            .metadata("sha256", sha256.as_str())
            .metadata("academy-kind", kind.path())
            .body(ByteStream::from(bytes))
            .send()
            .await
            .map_err(|error| StorageError::S3(error.to_string()))?;

        Ok(reference)
    }

    pub async fn get_blob(&self, reference: &BlobRef) -> Result<Vec<u8>, StorageError> {
        let response = self
            .client
            .get_object()
            .bucket(&self.config.bucket)
            .key(&reference.key)
            .send()
            .await
            .map_err(|error| StorageError::S3(error.to_string()))?;
        let bytes = response
            .body
            .collect()
            .await
            .map_err(|error| StorageError::S3(error.to_string()))?
            .into_bytes()
            .to_vec();
        let observed = ContentHash::of(&bytes);
        if observed != reference.sha256 || bytes.len() as u64 != reference.bytes {
            return Err(StorageError::Integrity {
                key: reference.key.clone(),
                expected: reference.sha256.clone(),
                observed,
            });
        }
        Ok(bytes)
    }
}

#[derive(Debug)]
pub enum StorageError {
    MissingEnvironment(&'static str),
    InvalidConfiguration(&'static str),
    InvalidContentHash(String),
    ContentAddressCollision(String),
    Integrity {
        key: String,
        expected: ContentHash,
        observed: ContentHash,
    },
    S3(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnvironment(name) => {
                write!(formatter, "missing environment variable {name}")
            }
            Self::InvalidConfiguration(message) => formatter.write_str(message),
            Self::InvalidContentHash(hash) => {
                write!(formatter, "invalid SHA-256 content hash: {hash}")
            }
            Self::ContentAddressCollision(key) => {
                write!(formatter, "content-addressed object mismatch at {key}")
            }
            Self::Integrity {
                key,
                expected,
                observed,
            } => write!(
                formatter,
                "integrity failure for {key}: expected {expected}, observed {observed}"
            ),
            Self::S3(message) => write!(formatter, "S3 operation failed: {message}"),
        }
    }
}

impl Error for StorageError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_hash_matches_sha256() {
        assert_eq!(
            ContentHash::of(b"abc").as_str(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn object_keys_are_partitioned_and_stable() {
        let config = S3StoreConfig::new("bucket", "ap-south-1", "/academy/v1/").unwrap();
        let hash = ContentHash::of(b"academy");
        assert_eq!(
            config.object_key(BlobKind::Checkpoint, &hash),
            format!(
                "academy/v1/checkpoints/{}/{}/{}",
                &hash.as_str()[..2],
                &hash.as_str()[2..4],
                hash
            )
        );
    }

    #[test]
    fn malformed_hashes_are_rejected() {
        assert!(ContentHash::parse("ABC").is_err());
        assert!(ContentHash::parse("z".repeat(64)).is_err());
    }
}
