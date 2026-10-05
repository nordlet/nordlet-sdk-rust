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

    pub async fn orders_create(
        &self,
        request: &OrdersCreateEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCreateEcommerceResponse, ApiError> {
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

    pub async fn orders_get(
        &self,
        request: &OrdersGetEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersGetEcommerceResponse, ApiError> {
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

    pub async fn orders_list(
        &self,
        request: &OrdersListEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersListEcommerceResponse, ApiError> {
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

    pub async fn orders_reserve(
        &self,
        request: &OrdersReserveEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersReserveEcommerceResponse, ApiError> {
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

    pub async fn orders_fulfill(
        &self,
        request: &OrdersFulfillEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersFulfillEcommerceResponse, ApiError> {
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

    pub async fn orders_cancel(
        &self,
        request: &OrdersCancelEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCancelEcommerceResponse, ApiError> {
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

    pub async fn products_list(
        &self,
        request: &ProductsListEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProductsListEcommerceResponse, ApiError> {
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

    pub async fn stock_list(
        &self,
        request: &StockListEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockListEcommerceResponse, ApiError> {
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
