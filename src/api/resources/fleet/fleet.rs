use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct FleetClient {
    pub http_client: HttpClient,
}

impl FleetClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_fleet_vehicles_create(
        &self,
        request: &PostV1FleetVehiclesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FleetVehiclesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fleet/vehicles/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_fleet_vehicles_update(
        &self,
        request: &PostV1FleetVehiclesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FleetVehiclesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fleet/vehicles/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_fleet_vehicles_get(
        &self,
        request: &PostV1FleetVehiclesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FleetVehiclesGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fleet/vehicles/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_fleet_vehicles_list(
        &self,
        request: &PostV1FleetVehiclesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FleetVehiclesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fleet/vehicles/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_fleet_assignments_create(
        &self,
        request: &PostV1FleetAssignmentsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FleetAssignmentsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fleet/assignments/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_fleet_assignments_end(
        &self,
        request: &PostV1FleetAssignmentsEndRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FleetAssignmentsEndResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fleet/assignments/end",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_fleet_assignments_list(
        &self,
        request: &PostV1FleetAssignmentsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FleetAssignmentsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fleet/assignments/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_fleet_natura_preview(
        &self,
        request: &PostV1FleetNaturaPreviewRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1FleetNaturaPreviewResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fleet/natura/preview",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
