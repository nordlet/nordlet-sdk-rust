use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BillingClient {
    pub http_client: HttpClient,
}

impl BillingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_billing_account_get(
        &self,
        request: &PostV1BillingAccountGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BillingAccountGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/account/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_billing_account_set_plan(
        &self,
        request: &PostV1BillingAccountSetPlanRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BillingAccountSetPlanResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/account/set-plan",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_billing_topup_create(
        &self,
        request: &PostV1BillingTopupCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BillingTopupCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/topup/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_billing_transactions_list(
        &self,
        request: &PostV1BillingTransactionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BillingTransactionsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/transactions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_billing_usage_list(
        &self,
        request: &PostV1BillingUsageListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BillingUsageListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/usage/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
