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

    /// List the audit log of the company: who did what to which entity, with the recorded change in `diff`. Filterable by `action`, `entity`, `entityId`, `actorType` and `actorId`; sortable by `createdAt` and `id`, newest first by default.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ListAuditRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAuditResponse, ApiError> {
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
