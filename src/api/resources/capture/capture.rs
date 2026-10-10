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
