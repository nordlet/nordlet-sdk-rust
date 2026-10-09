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

    pub async fn shifts_open(
        &self,
        request: &ShiftsOpenPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ShiftsOpenPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/shifts/open",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn shifts_get(
        &self,
        request: &ShiftsGetPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ShiftsGetPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/shifts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn shifts_list(
        &self,
        request: &ShiftsListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ShiftsListPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/shifts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn receipts_create(
        &self,
        request: &ReceiptsCreatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsCreatePosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/receipts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn receipts_list(
        &self,
        request: &ReceiptsListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsListPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/receipts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn receipts_get(
        &self,
        request: &ReceiptsGetPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsGetPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/receipts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn shifts_close(
        &self,
        request: &ShiftsClosePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ShiftsClosePosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/shifts/close",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
