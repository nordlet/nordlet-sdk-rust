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

    pub async fn orders_create(
        &self,
        request: &OrdersCreateCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCreateCashResponse, ApiError> {
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

    pub async fn orders_get(
        &self,
        request: &OrdersGetCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersGetCashResponse, ApiError> {
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

    pub async fn orders_list(
        &self,
        request: &OrdersListCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersListCashResponse, ApiError> {
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

    pub async fn balance(
        &self,
        request: &BalanceCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<BalanceCashResponse, ApiError> {
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

    pub async fn advance_holders_balances(
        &self,
        request: &AdvanceHoldersBalancesCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdvanceHoldersBalancesCashResponse, ApiError> {
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
