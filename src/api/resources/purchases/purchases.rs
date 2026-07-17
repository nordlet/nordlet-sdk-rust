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
}
