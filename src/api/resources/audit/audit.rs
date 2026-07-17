use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AuditClient {
    pub http_client: HttpClient,
}

impl AuditClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_audit_list(
        &self,
        request: &PostV1AuditListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AuditListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/audit/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
