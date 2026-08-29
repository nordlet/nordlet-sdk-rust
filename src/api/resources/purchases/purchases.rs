use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PurchasesClient {
    pub http_client: HttpClient,
}

impl PurchasesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_purchases_invoices_create(
        &self,
        request: &PostV1PurchasesInvoicesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesInvoicesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_invoices_get(
        &self,
        request: &PostV1PurchasesInvoicesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesInvoicesGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_invoices_update(
        &self,
        request: &PostV1PurchasesInvoicesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesInvoicesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_invoices_delete(
        &self,
        request: &PostV1PurchasesInvoicesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesInvoicesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_invoices_register(
        &self,
        request: &PostV1PurchasesInvoicesRegisterRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesInvoicesRegisterResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/register",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_invoices_list(
        &self,
        request: &PostV1PurchasesInvoicesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesInvoicesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_create(
        &self,
        request: &PostV1PurchasesOrdersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_update(
        &self,
        request: &PostV1PurchasesOrdersUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_get(
        &self,
        request: &PostV1PurchasesOrdersGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_list(
        &self,
        request: &PostV1PurchasesOrdersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_submit(
        &self,
        request: &PostV1PurchasesOrdersSubmitRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersSubmitResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/submit",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_approve(
        &self,
        request: &PostV1PurchasesOrdersApproveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersApproveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/approve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_reject(
        &self,
        request: &PostV1PurchasesOrdersRejectRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersRejectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/reject",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_cancel(
        &self,
        request: &PostV1PurchasesOrdersCancelRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersCancelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_close(
        &self,
        request: &PostV1PurchasesOrdersCloseRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersCloseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/close",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_orders_delete(
        &self,
        request: &PostV1PurchasesOrdersDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesOrdersDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_receipts_create(
        &self,
        request: &PostV1PurchasesReceiptsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesReceiptsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/receipts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_receipts_get(
        &self,
        request: &PostV1PurchasesReceiptsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesReceiptsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/receipts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_receipts_list(
        &self,
        request: &PostV1PurchasesReceiptsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesReceiptsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/receipts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_purchases_invoices_match(
        &self,
        request: &PostV1PurchasesInvoicesMatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PurchasesInvoicesMatchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/match",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
