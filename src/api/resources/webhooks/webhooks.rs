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

    /// Register a webhook endpoint for the given event types (`*` for every event). The URL must be an https address that resolves to a public network (422 otherwise). The signing secret is generated unless `secret` is given (at least 32 characters), and this call is the only one that returns it. Answers 503 when the platform encryption key is not configured.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's webhook subscriptions with their last delivery state. Filterable by `url` and `isActive`; sortable by `createdAt` and `url`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Replace the signing secret of a subscription and return the new one; this call is the only one that returns it. The secret is generated unless `secret` is given (at least 32 characters). For 24 hours `x-nordlet-signature-v2` carries two `v1=` values, one with the new secret and one with the previous one, so a receiver can switch secrets without refusing deliveries; `x-nordlet-signature` uses the new secret only. Answers 404 for an unknown subscription and 503 when the platform encryption key is not configured.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn subscriptions_rotate_secret(
        &self,
        request: &SubscriptionsRotateSecretWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionsRotateSecretWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks/subscriptions/rotate-secret",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change the URL, event types or `isActive` flag of a subscription; at least one field is required (400) and a new URL must be an https address that resolves to a public network (422). Setting `isActive` to true also clears `pausedAt` and `consecutiveFailures`, which is how a subscription paused after 5 failed deliveries in a row is switched back on.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete a webhook subscription.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's webhook deliveries, newest first. Filterable by `status`, `eventType` and `subscriptionId`; sortable by `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Send a delivery again now: its status goes back to `pending` with the attempt count and last error cleared. Answers 409 when the subscription is inactive.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
