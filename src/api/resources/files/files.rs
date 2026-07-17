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

    pub async fn post_v1_files_upload(
        &self,
        request: &PostV1FilesUploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FilesUploadResponse, ApiError> {
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

    pub async fn post_v1_files_get(
        &self,
        request: &PostV1FilesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FilesGetResponse, ApiError> {
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

    pub async fn post_v1_files_list(
        &self,
        request: &PostV1FilesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FilesListResponse, ApiError> {
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

    pub async fn post_v1_files_delete(
        &self,
        request: &PostV1FilesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FilesDeleteResponse, ApiError> {
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
