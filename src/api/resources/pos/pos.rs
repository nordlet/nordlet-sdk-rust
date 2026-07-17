use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PosClient {
    pub http_client: HttpClient,
}

impl PosClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_pos_devices_create(
        &self,
        request: &PostV1PosDevicesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PosDevicesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/devices/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_pos_devices_update(
        &self,
        request: &PostV1PosDevicesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PosDevicesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/devices/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_pos_devices_list(
        &self,
        request: &PostV1PosDevicesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PosDevicesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/devices/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_pos_reports_create(
        &self,
        request: &PostV1PosReportsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PosReportsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/reports/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_pos_reports_get(
        &self,
        request: &PostV1PosReportsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PosReportsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/reports/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_pos_reports_list(
        &self,
        request: &PostV1PosReportsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PosReportsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/reports/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
