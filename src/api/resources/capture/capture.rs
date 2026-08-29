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

    pub async fn read_a_vendor_bill_or_receipt_and_return_an_editable_purchase_invoice_draft(
        &self,
        request: &PostV1CaptureDocumentsUploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CaptureDocumentsUploadResponse, ApiError> {
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

    pub async fn re_read_a_stored_capture_replacing_the_previous_draft(
        &self,
        request: &PostV1CaptureDocumentsExtractRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CaptureDocumentsExtractResponse, ApiError> {
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

    pub async fn post_v1_capture_documents_get(
        &self,
        request: &PostV1CaptureDocumentsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CaptureDocumentsGetResponse, ApiError> {
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

    pub async fn post_v1_capture_documents_list(
        &self,
        request: &PostV1CaptureDocumentsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CaptureDocumentsListResponse, ApiError> {
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

    pub async fn post_v1_capture_documents_delete(
        &self,
        request: &PostV1CaptureDocumentsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CaptureDocumentsDeleteResponse, ApiError> {
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

    pub async fn save_the_reviewed_draft_as_a_purchase_invoice_and_attach_the_original_document(
        &self,
        request: &PostV1CaptureDocumentsConfirmRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CaptureDocumentsConfirmResponse, ApiError> {
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
