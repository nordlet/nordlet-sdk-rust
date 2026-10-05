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

    pub async fn vehicles_create(
        &self,
        request: &VehiclesCreateFleetRequest,
        options: Option<RequestOptions>,
    ) -> Result<VehiclesCreateFleetResponse, ApiError> {
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

    pub async fn vehicles_update(
        &self,
        request: &VehiclesUpdateFleetRequest,
        options: Option<RequestOptions>,
    ) -> Result<VehiclesUpdateFleetResponse, ApiError> {
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

    pub async fn vehicles_get(
        &self,
        request: &VehiclesGetFleetRequest,
        options: Option<RequestOptions>,
    ) -> Result<VehiclesGetFleetResponse, ApiError> {
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

    pub async fn vehicles_list(
        &self,
        request: &VehiclesListFleetRequest,
        options: Option<RequestOptions>,
    ) -> Result<VehiclesListFleetResponse, ApiError> {
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

    pub async fn assignments_create(
        &self,
        request: &AssignmentsCreateFleetRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssignmentsCreateFleetResponse, ApiError> {
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

    pub async fn assignments_end(
        &self,
        request: &AssignmentsEndFleetRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssignmentsEndFleetResponse, ApiError> {
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

    pub async fn assignments_list(
        &self,
        request: &AssignmentsListFleetRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssignmentsListFleetResponse, ApiError> {
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

    pub async fn natura_preview(
        &self,
        request: &NaturaPreviewFleetRequest,
        options: Option<RequestOptions>,
    ) -> Result<NaturaPreviewFleetResponse, ApiError> {
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
