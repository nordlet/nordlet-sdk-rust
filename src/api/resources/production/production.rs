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

    pub async fn post_v1_production_work_centers_create(
        &self,
        request: &PostV1ProductionWorkCentersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionWorkCentersCreateResponse, ApiError> {
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

    pub async fn post_v1_production_work_centers_update(
        &self,
        request: &PostV1ProductionWorkCentersUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionWorkCentersUpdateResponse, ApiError> {
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

    pub async fn post_v1_production_work_centers_list(
        &self,
        request: &PostV1ProductionWorkCentersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionWorkCentersListResponse, ApiError> {
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

    pub async fn post_v1_production_routings_create(
        &self,
        request: &PostV1ProductionRoutingsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionRoutingsCreateResponse, ApiError> {
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

    pub async fn post_v1_production_routings_get(
        &self,
        request: &PostV1ProductionRoutingsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionRoutingsGetResponse, ApiError> {
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

    pub async fn post_v1_production_routings_list(
        &self,
        request: &PostV1ProductionRoutingsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionRoutingsListResponse, ApiError> {
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

    pub async fn post_v1_production_maintenance_create(
        &self,
        request: &PostV1ProductionMaintenanceCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionMaintenanceCreateResponse, ApiError> {
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

    pub async fn post_v1_production_maintenance_complete(
        &self,
        request: &PostV1ProductionMaintenanceCompleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionMaintenanceCompleteResponse, ApiError> {
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

    pub async fn post_v1_production_maintenance_cancel(
        &self,
        request: &PostV1ProductionMaintenanceCancelRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionMaintenanceCancelResponse, ApiError> {
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

    pub async fn post_v1_production_maintenance_list(
        &self,
        request: &PostV1ProductionMaintenanceListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionMaintenanceListResponse, ApiError> {
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

    pub async fn post_v1_production_boms_create(
        &self,
        request: &PostV1ProductionBomsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionBomsCreateResponse, ApiError> {
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

    pub async fn post_v1_production_boms_get(
        &self,
        request: &PostV1ProductionBomsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionBomsGetResponse, ApiError> {
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

    pub async fn post_v1_production_boms_list(
        &self,
        request: &PostV1ProductionBomsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionBomsListResponse, ApiError> {
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

    pub async fn post_v1_production_orders_create(
        &self,
        request: &PostV1ProductionOrdersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersCreateResponse, ApiError> {
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

    pub async fn post_v1_production_orders_record_operation(
        &self,
        request: &PostV1ProductionOrdersRecordOperationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersRecordOperationResponse, ApiError> {
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

    pub async fn post_v1_production_quality_checks_add(
        &self,
        request: &PostV1ProductionQualityChecksAddRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionQualityChecksAddResponse, ApiError> {
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

    pub async fn post_v1_production_quality_checks_record(
        &self,
        request: &PostV1ProductionQualityChecksRecordRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionQualityChecksRecordResponse, ApiError> {
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

    pub async fn post_v1_production_quality_checks_list(
        &self,
        request: &PostV1ProductionQualityChecksListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionQualityChecksListResponse, ApiError> {
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

    pub async fn post_v1_production_orders_complete(
        &self,
        request: &PostV1ProductionOrdersCompleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersCompleteResponse, ApiError> {
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

    pub async fn post_v1_production_orders_get(
        &self,
        request: &PostV1ProductionOrdersGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersGetResponse, ApiError> {
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

    pub async fn post_v1_production_orders_list(
        &self,
        request: &PostV1ProductionOrdersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProductionOrdersListResponse, ApiError> {
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
