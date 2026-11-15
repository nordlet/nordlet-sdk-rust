use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ReviewClient {
    pub http_client: HttpClient,
}

impl ReviewClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Read-only. Lists, each with its `kind`, the `id` of the record, a `message` and the `fix` (the call or field that resolves it): `draft_warning` (a draft purchase or sale document with a warning of `purchases/invoices/create` or `sales/invoices/create`, one item per warning, with its `code`), `bank_transaction_unbooked` (a bank line with the status `new`), `partner_vat_invalid` (a partner whose VAT number failed its VIES check) and `filing_settings_missing` (a filing due in the window of `account/companies/context` whose settings are missing, with the `declarations/configs/update` call; the id is the calendar key). Each kind lists at most 200 records, the newest first. An empty list means nothing needs a look.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn queue_list(
        &self,
        request: &QueueListReviewRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueueListReviewResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/review/queue/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
