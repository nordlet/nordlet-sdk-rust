use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct MigrationClient {
    pub http_client: HttpClient,
}

impl MigrationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Runs every check the import runs (accounts, partners, balances, open invoices, assets, stock) and returns the same summary and warnings, then rolls everything back. Nothing is stored.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn check_a_historical_books_package_without_writing_anything(
        &self,
        request: &PostV1MigrationBooksValidateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1MigrationBooksValidateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/migration/books/validate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Brings a company over from another system in one call: chart of accounts, partners, items, opening balances (or the full journal history), open customer and supplier invoices, fixed assets with their accumulated depreciation, and stock on hand. The whole package is written in one database transaction — if any row fails, nothing is stored.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_historical_books_from_a_previous_accounting_system(
        &self,
        request: &PostV1MigrationBooksImportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1MigrationBooksImportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/migration/books/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
