use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DocumentSeriesClient {
    pub http_client: HttpClient,
}

impl DocumentSeriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create(
        &self,
        request: &CreateDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn update(
        &self,
        request: &UpdateDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        request: &GetDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        request: &DeleteDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn list(
        &self,
        request: &ListDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
