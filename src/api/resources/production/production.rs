use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ProductionClient {
    pub http_client: HttpClient,
}

impl ProductionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_production_boms_create(
        &self,
        request: &PostV1ProductionBomsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionBomsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_production_boms_get(
        &self,
        request: &PostV1ProductionBomsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionBomsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_production_boms_list(
        &self,
        request: &PostV1ProductionBomsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionBomsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_production_orders_create(
        &self,
        request: &PostV1ProductionOrdersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_production_orders_complete(
        &self,
        request: &PostV1ProductionOrdersCompleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersCompleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/complete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_production_orders_get(
        &self,
        request: &PostV1ProductionOrdersGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_production_orders_list(
        &self,
        request: &PostV1ProductionOrdersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
