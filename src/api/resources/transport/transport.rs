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

    pub async fn post_v1_transport_waybills_create(
        &self,
        request: &PostV1TransportWaybillsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1TransportWaybillsCreateResponse, ApiError> {
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

    pub async fn post_v1_transport_waybills_update(
        &self,
        request: &PostV1TransportWaybillsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1TransportWaybillsUpdateResponse, ApiError> {
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

    pub async fn post_v1_transport_waybills_issue(
        &self,
        request: &PostV1TransportWaybillsIssueRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1TransportWaybillsIssueResponse, ApiError> {
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

    pub async fn post_v1_transport_waybills_cancel(
        &self,
        request: &PostV1TransportWaybillsCancelRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1TransportWaybillsCancelResponse, ApiError> {
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

    pub async fn post_v1_transport_waybills_get(
        &self,
        request: &PostV1TransportWaybillsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1TransportWaybillsGetResponse, ApiError> {
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

    pub async fn post_v1_transport_waybills_list(
        &self,
        request: &PostV1TransportWaybillsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1TransportWaybillsListResponse, ApiError> {
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
