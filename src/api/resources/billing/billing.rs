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

    pub async fn account_get(
        &self,
        request: &AccountGetBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountGetBillingResponse, ApiError> {
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

    pub async fn account_set_plan(
        &self,
        request: &AccountSetPlanBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountSetPlanBillingResponse, ApiError> {
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

    pub async fn topup_create(
        &self,
        request: &TopupCreateBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<TopupCreateBillingResponse, ApiError> {
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

    pub async fn portal_create(
        &self,
        request: &PortalCreateBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<PortalCreateBillingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/billing/portal/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn transactions_list(
        &self,
        request: &TransactionsListBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsListBillingResponse, ApiError> {
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

    pub async fn usage_list(
        &self,
        request: &UsageListBillingRequest,
        options: Option<RequestOptions>,
    ) -> Result<UsageListBillingResponse, ApiError> {
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
