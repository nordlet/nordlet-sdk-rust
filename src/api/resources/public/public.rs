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

    /// Store a request for an integration: the integration asked for and the sender's name, company, e-mail and details. When `website` is filled in, the call answers `received` without storing anything.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn integration_requests(
        &self,
        request: &IntegrationRequestsPublicRequest,
        options: Option<RequestOptions>,
    ) -> Result<IntegrationRequestsPublicResponse, ApiError> {
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

    /// Redirect (302) to the company's payment page for the issued invoice that carries this payment token, rendered from the template set with `sales/invoices/payment-settings/update` with the invoice number, the amount still unpaid, the currency, the token, the invoice id and the customer name (see `sales/invoices/payment-link`). Answers 404 when the token matches no issued invoice or the company has no template, and 409 when the invoice is already paid.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn pay(&self, token: &str, options: Option<RequestOptions>) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/public/pay/{}", token),
                None,
                None,
                options,
            )
            .await
    }
}
