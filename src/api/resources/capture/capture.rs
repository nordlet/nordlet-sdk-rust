use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CaptureClient {
    pub http_client: HttpClient,
}

impl CaptureClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Parse a capture received over Peppol from its stored document again and replace its draft (supplier, totals, lines, matched partner). Refused with 422 for a capture that was not received over Peppol and with 409 once the capture is linked to a purchase invoice.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn documents_extract(
        &self,
        request: &DocumentsExtractCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentsExtractCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/documents/extract",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one document received over Peppol (a capture) with its draft: supplier, dates, totals and lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn documents_get(
        &self,
        request: &DocumentsGetCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentsGetCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/documents/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the documents received over Peppol (captures) with their drafts. Filterable by `status`, `matchedPartnerId` and `purchaseInvoiceId`; sortable by `createdAt` and `updatedAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn documents_list(
        &self,
        request: &DocumentsListCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentsListCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/documents/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a capture and its stored file (409 once it is linked to a purchase invoice).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn documents_delete(
        &self,
        request: &DocumentsDeleteCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentsDeleteCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/documents/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates the purchase invoice (or credit note, see `type`) from `lines`. Lines with the opposite sign go in `oppositeLines` and are saved as a second document of the opposite type for the same supplier: a purchase credit note against the new invoice, or a purchase invoice next to the new credit note. It is numbered `oppositeDocumentNumber`, by default the document number followed by "-CR" (credit note) or "-INV" (invoice).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn documents_confirm(
        &self,
        request: &DocumentsConfirmCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentsConfirmCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/documents/confirm",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
