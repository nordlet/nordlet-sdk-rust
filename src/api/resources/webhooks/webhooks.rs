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

    pub async fn post_v1_webhooks_subscriptions_create(
        &self,
        request: &PostV1WebhooksSubscriptionsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1WebhooksSubscriptionsCreateResponse, ApiError> {
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

    pub async fn post_v1_webhooks_subscriptions_list(
        &self,
        request: &PostV1WebhooksSubscriptionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1WebhooksSubscriptionsListResponse, ApiError> {
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

    pub async fn post_v1_webhooks_subscriptions_update(
        &self,
        request: &PostV1WebhooksSubscriptionsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1WebhooksSubscriptionsUpdateResponse, ApiError> {
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

    pub async fn post_v1_webhooks_subscriptions_delete(
        &self,
        request: &PostV1WebhooksSubscriptionsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1WebhooksSubscriptionsDeleteResponse, ApiError> {
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

    pub async fn post_v1_webhooks_deliveries_list(
        &self,
        request: &PostV1WebhooksDeliveriesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1WebhooksDeliveriesListResponse, ApiError> {
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

    pub async fn post_v1_webhooks_deliveries_redeliver(
        &self,
        request: &PostV1WebhooksDeliveriesRedeliverRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1WebhooksDeliveriesRedeliverResponse, ApiError> {
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
