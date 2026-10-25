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

    /// Redirect (302) to the company's payment page for the issued invoice that carries this payment token, rendered from the template set with `sales/invoices/payment-settings/update` with the invoice number, the amount still unpaid, the currency, the token, the invoice id and the customer name (see `sales/invoices/payment-link`). Answers 404 when the token matches no issued invoice, the invoice has `payLinkEnabled` false or the company has no template, and 409 when the invoice is already paid.
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

    /// Return the issued sale document that carries this customer token (`token` of `sales/invoices/view`), in the shape of `sales/invoices/view` without the token. Answers 404 when the token matches no issued document.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_get(
        &self,
        request: &InvoicesGetPublicRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesGetPublicResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/public/invoices/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Open a Stripe Checkout page on the company's connected Stripe account for the amount still unpaid on the issued invoice that carries this customer token, and return its `url`. Stripe sends the customer back to the public invoice page. Answers 404 when the token matches no issued invoice, the invoice has `payLinkEnabled` false or the company has no Stripe account that can take charges, 409 when the invoice is already paid and 422 when Stripe cannot take the amount in the invoice currency.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_checkout(
        &self,
        request: &InvoicesCheckoutPublicRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesCheckoutPublicResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/public/invoices/checkout",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return the PDF of the issued sale document that carries this customer token, base64-encoded, as `sales/invoices/pdf` renders it in the document's own language, English when it has none. Answers 404 when the token matches no issued document.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_pdf(
        &self,
        request: &InvoicesPdfPublicRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesPdfPublicResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/public/invoices/pdf",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
