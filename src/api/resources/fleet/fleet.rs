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

    /// Register a vehicle. The plate number is stored in upper case and must be unique in the company (409 otherwise); `fixedAssetId` must name an existing fixed asset. Uploaded `documents` are attached to the vehicle.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change a vehicle's details or set its `status` to `active`, `sold` or `scrapped`; omitted fields keep their value. A plate number already used by another vehicle is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Read one vehicle with its current assignment, when an employee holds it today.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List vehicles with their current assignment. Filterable by `plateNumber`, `make`, `model` and `status`; sortable by `plateNumber`, `make` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Assign a vehicle to an employee from `fromDate`, open-ended or until `toDate`. A period that overlaps another assignment of the same vehicle is refused with 409, a `toDate` before `fromDate` with 422. `privateUse` makes the car a benefit in kind in `fleet/natura/preview`, and `employerPaysFuel` picks its rate.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// End an open assignment on `toDate`. An assignment that already has an end date is refused with 409, a `toDate` before its `fromDate` with 422.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List vehicle assignments with the plate number and the employee's name. Filterable by `vehicleId`, `employeeId` and `privateUse`; sortable by `fromDate` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Preview the car benefit in kind of a month for a Lithuanian company: every assignment with `privateUse` that overlaps the month is valued at 0.75 % of the vehicle's market value when the employer pays the fuel, else 0.70 %, one row per employee and vehicle, with the total. A company registered in another country is refused with 422. The same amounts are added to a payroll run created with `includeNatura` (`payroll/runs/create`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
