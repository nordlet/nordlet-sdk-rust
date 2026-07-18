use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SalesClient {
    pub http_client: HttpClient,
}

impl SalesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_sales_invoices_create(
        &self,
        request: &PostV1SalesInvoicesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_get(
        &self,
        request: &PostV1SalesInvoicesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_pdf(
        &self,
        request: &PostV1SalesInvoicesPdfRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesPdfResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/pdf",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_send(
        &self,
        request: &PostV1SalesInvoicesSendRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesSendResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_peppol_xml(
        &self,
        request: &PostV1SalesInvoicesPeppolXmlRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesPeppolXmlResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/peppol-xml",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_peppol_send(
        &self,
        request: &PostV1SalesInvoicesPeppolSendRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesPeppolSendResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/peppol-send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_update(
        &self,
        request: &PostV1SalesInvoicesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_delete(
        &self,
        request: &PostV1SalesInvoicesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_issue(
        &self,
        request: &PostV1SalesInvoicesIssueRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesIssueResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/issue",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_recognition_schedules_list(
        &self,
        request: &PostV1SalesRecognitionSchedulesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRecognitionSchedulesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition-schedules/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_apply_advance(
        &self,
        request: &PostV1SalesInvoicesApplyAdvanceRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesApplyAdvanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/apply-advance",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_invoices_list(
        &self,
        request: &PostV1SalesInvoicesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesInvoicesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_acts_create(
        &self,
        request: &PostV1SalesActsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesActsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_acts_update(
        &self,
        request: &PostV1SalesActsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesActsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_acts_issue(
        &self,
        request: &PostV1SalesActsIssueRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesActsIssueResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/issue",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_acts_cancel(
        &self,
        request: &PostV1SalesActsCancelRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesActsCancelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_acts_get(
        &self,
        request: &PostV1SalesActsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesActsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_acts_list(
        &self,
        request: &PostV1SalesActsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesActsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_acts_pdf(
        &self,
        request: &PostV1SalesActsPdfRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesActsPdfResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/pdf",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_recognition_compute(
        &self,
        request: &PostV1SalesRecognitionComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRecognitionComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_recognition_run(
        &self,
        request: &PostV1SalesRecognitionRunRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRecognitionRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/run",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_recognition_progress(
        &self,
        request: &PostV1SalesRecognitionProgressRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRecognitionProgressResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/progress",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Apply an IFRS 15 contract modification to a deferred invoice line. Prospective: cancel the pending schedule and respread the unrecognized remainder over the new terms. Cumulative catch-up (ratable only): recompute revenue as if the new terms applied from the start and post the difference immediately.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_sales_recognition_modify(
        &self,
        request: &PostV1SalesRecognitionModifyRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRecognitionModifyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/modify",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_recognition_runs_list(
        &self,
        request: &PostV1SalesRecognitionRunsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRecognitionRunsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/runs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_recognition_summary(
        &self,
        request: &PostV1SalesRecognitionSummaryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRecognitionSummaryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/summary",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_refund_liability_list(
        &self,
        request: &PostV1SalesRefundLiabilityListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRefundLiabilityListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/refund-liability/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_sales_refund_liability_true_up(
        &self,
        request: &PostV1SalesRefundLiabilityTrueUpRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1SalesRefundLiabilityTrueUpResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/refund-liability/true-up",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
