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
}
