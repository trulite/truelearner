use academy_storage::{BlobKind, S3AcademyStore, S3StoreConfig};

const PAYLOAD: &[u8] = b"truelearner-academy-s3-smoke-v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = S3AcademyStore::load(S3StoreConfig::from_env()?).await;
    let reference = store
        .put_blob(BlobKind::Manifest, PAYLOAD, "application/octet-stream")
        .await?;
    let restored = store.get_blob(&reference).await?;
    if restored != PAYLOAD {
        return Err("S3 round trip changed bytes".into());
    }
    println!("ACADEMY_S3_SMOKE_OK key={}", reference.key);
    Ok(())
}
