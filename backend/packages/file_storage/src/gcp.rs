use bytes::Bytes;
use environment::Environment;
use errors::Result;
use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::{
        upload::{UploadObjectRequest, UploadType},
        Object,
    },
};

pub struct GoogleCloudStorage {
    client: Client,
}

impl GoogleCloudStorage {
    // Initialize a new GoogleCloudStorage client
    pub async fn new() -> Result<Self> {
        // Load credentials from a file
        let env = Environment::new();
        let credentials_path = env.gcp_credentials;
        let cred = CredentialsFile::new_from_file(credentials_path).await?;

        // Create a Google Cloud Storage client with the credentials
        let config = ClientConfig::default().with_credentials(cred).await?;
        let client = Client::new(config);

        Ok(GoogleCloudStorage { client })
    }

    // Upload a file and set its permission to public
    pub async fn upload_file(
        &self,
        content: Bytes,
        content_type: &str,
        file_name: &str,
    ) -> Result<String> {
        let env = Environment::new();
        let bucket = env.storage_bucket;
        let storage_api = env.gcp_storage_api;

        // Prepare the file upload request using the Multipart upload type
        let upload_type = UploadType::Multipart(Box::new(Object {
            name: file_name.to_string(),
            content_type: Some(content_type.to_string()),
            ..Default::default()
        }));

        // Upload the file to the GCS bucket
        self.client
            .upload_object(
                &UploadObjectRequest {
                    bucket: bucket.to_string(), // Specify your bucket name
                    ..Default::default()
                },
                content,
                &upload_type,
            )
            .await?;

        // Construct the public URL for the uploaded file
        let url = format!("{}/{}/{}", storage_api, bucket, file_name);

        Ok(url) // Return the URL of the uploaded file
    }
}
