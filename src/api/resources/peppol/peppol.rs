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

    /// Apply one event: a delivery status event updates the Peppol status of the sale invoice with that message id and, when the status changes, emits `sale_invoice.peppol_delivered`, `sale_invoice.peppol_rejected` or `sale_invoice.peppol_failed`; a received-document event fetches the document from the access point, stores it as a document capture with the matched supplier, emits `document_capture.peppol_received` and marks it read at the access point (a document already stored is skipped). `handled` is false when the event matched no invoice or document. Answers 404 when no company uses that provider account, 401 when the signature does not match.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
