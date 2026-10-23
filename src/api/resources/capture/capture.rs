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

    /// Return the company's document capture settings: whether intake by e-mail is enabled, whether e-mailed documents are read on arrival, the company's intake e-mail address and whether a reading provider is configured. The intake token behind the address is created on the first call.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_get(
        &self,
        request: &SettingsGetCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsGetCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/settings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Switch `intakeEnabled` (documents sent to the intake address are accepted) and `captureAutoExtract` (they are read on arrival) on or off; at least one field is required.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_update(
        &self,
        request: &SettingsUpdateCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsUpdateCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/settings/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replace the company's intake token, so the previous intake e-mail address stops working and the settings come back with the new one.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_regenerate_intake(
        &self,
        request: &SettingsRegenerateIntakeCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsRegenerateIntakeCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/settings/regenerate-intake",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Endpoint for the inbound e-mail provider (Postmark field names or the generic `to`, `from`, `subject` and `attachments` form). It needs the `x-inbound-secret` header and finds the company by the `intake+<token>@` recipient address (404 when unknown, 403 when intake is disabled). Every PDF, image, Word or PowerPoint attachment up to 10 MB becomes a `pending` capture (`accepted`), anything else is `skipped`; when the company has automatic reading on, is not on trial and a reading provider is configured, each capture is read at once.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn inbound_email(
        &self,
        request: &InboundEmailCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<InboundEmailCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/inbound-email",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Upload a supplier document (PDF, image, Word or PowerPoint, up to 10 MB; 422 for any other type) as a capture and read it at once. The result holds the extracted supplier, dates, totals and lines, the partner matched by VAT code, then partner code, then name, the raw text, and `status` `extracted` or `failed` with `error`. Needs a configured reading provider, is refused with 402 for a company on trial until its first top-up, and counts every page read as `ocr_page` usage. The draft is reviewed and saved with `capture/documents/confirm`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn documents_upload(
        &self,
        request: &DocumentsUploadCaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentsUploadCaptureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/capture/documents/upload",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read a stored capture again and replace its draft (supplier, totals, lines, matched partner). A capture received over Peppol is parsed from its document again without a reading provider; any other needs the provider and, for a company on trial, the first top-up (402). Refused with 409 once the capture is linked to a purchase invoice.
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

    /// Return one capture with its extraction and raw text.
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

    /// List captures without raw text. Filterable by `status`, `matchedPartnerId` and `purchaseInvoiceId`; sortable by `createdAt` and `updatedAt`.
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
