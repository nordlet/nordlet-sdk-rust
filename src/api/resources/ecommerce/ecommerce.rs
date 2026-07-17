use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct EcommerceClient {
    pub http_client: HttpClient,
}

impl EcommerceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_ecommerce_orders_create(
        &self,
        request: &PostV1EcommerceOrdersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1EcommerceOrdersCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_ecommerce_orders_get(
        &self,
        request: &PostV1EcommerceOrdersGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1EcommerceOrdersGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_ecommerce_orders_list(
        &self,
        request: &PostV1EcommerceOrdersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1EcommerceOrdersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_ecommerce_orders_reserve(
        &self,
        request: &PostV1EcommerceOrdersReserveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1EcommerceOrdersReserveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/reserve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_ecommerce_orders_fulfill(
        &self,
        request: &PostV1EcommerceOrdersFulfillRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1EcommerceOrdersFulfillResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/fulfill",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_ecommerce_orders_cancel(
        &self,
        request: &PostV1EcommerceOrdersCancelRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1EcommerceOrdersCancelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_ecommerce_products_list(
        &self,
        request: &PostV1EcommerceProductsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1EcommerceProductsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/products/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_ecommerce_stock_list(
        &self,
        request: &PostV1EcommerceStockListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1EcommerceStockListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/stock/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
