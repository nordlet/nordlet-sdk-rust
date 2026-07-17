use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CashClient {
    pub http_client: HttpClient,
}

impl CashClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_cash_orders_create(
        &self,
        request: &PostV1CashOrdersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CashOrdersCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_cash_orders_get(
        &self,
        request: &PostV1CashOrdersGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CashOrdersGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_cash_orders_list(
        &self,
        request: &PostV1CashOrdersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CashOrdersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_cash_balance(
        &self,
        request: &PostV1CashBalanceRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CashBalanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/balance",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_cash_advance_holders_balances(
        &self,
        request: &PostV1CashAdvanceHoldersBalancesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CashAdvanceHoldersBalancesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/advance-holders/balances",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
