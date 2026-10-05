use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OperationTypesClient {
    pub http_client: HttpClient,
}

impl OperationTypesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create(
        &self,
        request: &CreateOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn update(
        &self,
        request: &UpdateOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        request: &GetOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        request: &DeleteOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn list(
        &self,
        request: &ListOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
