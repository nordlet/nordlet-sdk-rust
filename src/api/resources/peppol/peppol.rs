use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PeppolClient {
    pub http_client: HttpClient,
}

impl PeppolClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Look a receiver up on the Peppol network (SML and SMP) and say which Peppol BIS Billing 3.0 documents it accepts. Give `partnerId` to look up a partner by its Peppol ID, VAT code or registration code, or `participantId` as "<scheme>:<identifier>". Works without an access point.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn participants_lookup(
        &self,
        request: &ParticipantsLookupPeppolRequest,
        options: Option<RequestOptions>,
    ) -> Result<ParticipantsLookupPeppolResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/peppol/participants/lookup",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn webhooks(
        &self,
        provider: &WebhooksPeppolRequestProvider,
        company_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksPeppolResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/peppol/webhooks/{}/{}", provider, company_id),
                None,
                None,
                options,
            )
            .await
    }
}
