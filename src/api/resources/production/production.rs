use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ProductionClient {
    pub http_client: HttpClient,
}

impl ProductionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn work_centers_create(
        &self,
        request: &WorkCentersCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkCentersCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/work-centers/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn work_centers_update(
        &self,
        request: &WorkCentersUpdateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkCentersUpdateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/work-centers/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn work_centers_list(
        &self,
        request: &WorkCentersListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkCentersListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/work-centers/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn routings_create(
        &self,
        request: &RoutingsCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoutingsCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/routings/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn routings_get(
        &self,
        request: &RoutingsGetProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoutingsGetProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/routings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn routings_list(
        &self,
        request: &RoutingsListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoutingsListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/routings/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn maintenance_create(
        &self,
        request: &MaintenanceCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<MaintenanceCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/maintenance/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn maintenance_complete(
        &self,
        request: &MaintenanceCompleteProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<MaintenanceCompleteProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/maintenance/complete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn maintenance_cancel(
        &self,
        request: &MaintenanceCancelProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<MaintenanceCancelProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/maintenance/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn maintenance_list(
        &self,
        request: &MaintenanceListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<MaintenanceListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/maintenance/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn boms_create(
        &self,
        request: &BomsCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<BomsCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn boms_get(
        &self,
        request: &BomsGetProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<BomsGetProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn boms_list(
        &self,
        request: &BomsListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<BomsListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn orders_create(
        &self,
        request: &OrdersCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn orders_record_operation(
        &self,
        request: &OrdersRecordOperationProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersRecordOperationProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/record-operation",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn quality_checks_add(
        &self,
        request: &QualityChecksAddProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<QualityChecksAddProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/quality-checks/add",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn quality_checks_record(
        &self,
        request: &QualityChecksRecordProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<QualityChecksRecordProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/quality-checks/record",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn quality_checks_list(
        &self,
        request: &QualityChecksListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<QualityChecksListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/quality-checks/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn orders_complete(
        &self,
        request: &OrdersCompleteProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCompleteProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/complete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn orders_get(
        &self,
        request: &OrdersGetProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersGetProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn orders_list(
        &self,
        request: &OrdersListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
