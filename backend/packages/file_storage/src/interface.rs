use super::gcp::GoogleCloudStorage;
use axum::async_trait;
use bytes::Bytes;
use errors::Result;

#[derive(Clone)]
pub enum StoragePlatform {
    Google,
    Amazon,
    // Add other storage platforms here, e.g., Azure
}

/* Trait for file storage operations */
#[async_trait]
pub trait FileStorageInterface {
    /* Method to upload a file to the storage */
    async fn upload(
        &self,
        content: Bytes,
        content_type: &str,
        storage_destination_path: &str,
    ) -> Result<String>;
}

#[derive(Clone)]
pub struct FileStorage {
    pub platform: StoragePlatform,
}

#[async_trait]
impl FileStorageInterface for FileStorage {
    async fn upload(
        &self,
        content: Bytes,
        content_type: &str,
        storage_destination_path: &str,
    ) -> Result<String> {
        // Implementation goes here
        match &self.platform {
            StoragePlatform::Google => {
                let gcp_client = GoogleCloudStorage::new().await?;
                let url = gcp_client
                    .upload_file(content, content_type, storage_destination_path)
                    .await?;
                Ok(url)
            }
            StoragePlatform::Amazon => Ok("Not Ready Yet".to_string()),
        }
    }
}
