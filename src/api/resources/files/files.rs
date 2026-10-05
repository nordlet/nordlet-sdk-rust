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
