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

    pub async fn devices_create(
        &self,
        request: &DevicesCreatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<DevicesCreatePosResponse, ApiError> {
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

    pub async fn devices_update(
        &self,
        request: &DevicesUpdatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<DevicesUpdatePosResponse, ApiError> {
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

    pub async fn devices_list(
        &self,
        request: &DevicesListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<DevicesListPosResponse, ApiError> {
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

    pub async fn reports_create(
        &self,
        request: &ReportsCreatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportsCreatePosResponse, ApiError> {
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

    pub async fn reports_get(
        &self,
        request: &ReportsGetPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportsGetPosResponse, ApiError> {
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

    pub async fn reports_list(
        &self,
        request: &ReportsListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportsListPosResponse, ApiError> {
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
