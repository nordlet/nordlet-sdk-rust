use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn subscriptions_create(
        &self,
        request: &SubscriptionsCreateWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionsCreateWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks/subscriptions/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn subscriptions_list(
        &self,
        request: &SubscriptionsListWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionsListWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks/subscriptions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn subscriptions_update(
        &self,
        request: &SubscriptionsUpdateWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionsUpdateWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks/subscriptions/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn subscriptions_delete(
        &self,
        request: &SubscriptionsDeleteWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionsDeleteWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks/subscriptions/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn deliveries_list(
        &self,
        request: &DeliveriesListWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeliveriesListWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks/deliveries/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn deliveries_redeliver(
        &self,
        request: &DeliveriesRedeliverWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeliveriesRedeliverWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks/deliveries/redeliver",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
