use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct FilesClient {
    pub http_client: HttpClient,
}

impl FilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Store a base64-encoded file of up to 10 MB (413 above that) under an `entity` and optional `entityId`, and return its record with the SHA-256 of the content and its `storageKey`, which other calls take as `documentRef`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upload(
        &self,
        request: &UploadFilesRequest,
        options: Option<RequestOptions>,
    ) -> Result<UploadFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/files/upload",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return a file record with its content, base64-encoded.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        request: &GetFilesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/files/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the company's files. Filterable by `entity`, `entityId` and `fileName`; sortable by `createdAt`, `fileName` and `sizeBytes`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ListFilesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/files/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a file record and its stored content.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        request: &DeleteFilesRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/files/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
