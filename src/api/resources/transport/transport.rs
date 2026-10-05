use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TransportClient {
    pub http_client: HttpClient,
}

impl TransportClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn waybills_create(
        &self,
        request: &WaybillsCreateTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsCreateTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn waybills_update(
        &self,
        request: &WaybillsUpdateTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsUpdateTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn waybills_issue(
        &self,
        request: &WaybillsIssueTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsIssueTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/issue",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn waybills_cancel(
        &self,
        request: &WaybillsCancelTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsCancelTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn waybills_get(
        &self,
        request: &WaybillsGetTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsGetTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn waybills_list(
        &self,
        request: &WaybillsListTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsListTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
