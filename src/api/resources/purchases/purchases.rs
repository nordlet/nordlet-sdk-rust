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

    pub async fn invoices_create(
        &self,
        request: &InvoicesCreatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesCreatePurchasesResponse, ApiError> {
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

    pub async fn invoices_get(
        &self,
        request: &InvoicesGetPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesGetPurchasesResponse, ApiError> {
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

    pub async fn invoices_update(
        &self,
        request: &InvoicesUpdatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesUpdatePurchasesResponse, ApiError> {
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

    pub async fn invoices_delete(
        &self,
        request: &InvoicesDeletePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesDeletePurchasesResponse, ApiError> {
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

    pub async fn invoices_register(
        &self,
        request: &InvoicesRegisterPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesRegisterPurchasesResponse, ApiError> {
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

    pub async fn invoices_list(
        &self,
        request: &InvoicesListPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesListPurchasesResponse, ApiError> {
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

    pub async fn orders_create(
        &self,
        request: &OrdersCreatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCreatePurchasesResponse, ApiError> {
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

    pub async fn orders_update(
        &self,
        request: &OrdersUpdatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersUpdatePurchasesResponse, ApiError> {
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

    pub async fn orders_get(
        &self,
        request: &OrdersGetPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersGetPurchasesResponse, ApiError> {
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

    pub async fn orders_list(
        &self,
        request: &OrdersListPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersListPurchasesResponse, ApiError> {
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

    pub async fn orders_submit(
        &self,
        request: &OrdersSubmitPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersSubmitPurchasesResponse, ApiError> {
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

    pub async fn orders_approve(
        &self,
        request: &OrdersApprovePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersApprovePurchasesResponse, ApiError> {
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

    pub async fn orders_reject(
        &self,
        request: &OrdersRejectPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersRejectPurchasesResponse, ApiError> {
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

    pub async fn orders_cancel(
        &self,
        request: &OrdersCancelPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCancelPurchasesResponse, ApiError> {
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

    pub async fn orders_close(
        &self,
        request: &OrdersClosePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersClosePurchasesResponse, ApiError> {
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

    pub async fn orders_delete(
        &self,
        request: &OrdersDeletePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersDeletePurchasesResponse, ApiError> {
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

    pub async fn receipts_create(
        &self,
        request: &ReceiptsCreatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsCreatePurchasesResponse, ApiError> {
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

    pub async fn receipts_get(
        &self,
        request: &ReceiptsGetPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsGetPurchasesResponse, ApiError> {
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

    pub async fn receipts_list(
        &self,
        request: &ReceiptsListPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsListPurchasesResponse, ApiError> {
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

    pub async fn invoices_match(
        &self,
        request: &InvoicesMatchPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesMatchPurchasesResponse, ApiError> {
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
