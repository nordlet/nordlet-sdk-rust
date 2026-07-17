use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PublicClient {
    pub http_client: HttpClient,
}

impl PublicClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_public_integration_requests(
        &self,
        request: &PostV1PublicIntegrationRequestsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PublicIntegrationRequestsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/public/integration-requests",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
