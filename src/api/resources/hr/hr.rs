use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct HrClient {
    pub http_client: HttpClient,
}

impl HrClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_hr_positions_create(
        &self,
        request: &PostV1HrPositionsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrPositionsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/positions/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_positions_update(
        &self,
        request: &PostV1HrPositionsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrPositionsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/positions/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_positions_list(
        &self,
        request: &PostV1HrPositionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrPositionsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/positions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_create(
        &self,
        request: &PostV1HrEmployeesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_update(
        &self,
        request: &PostV1HrEmployeesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_get(
        &self,
        request: &PostV1HrEmployeesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_list(
        &self,
        request: &PostV1HrEmployeesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_contracts_create(
        &self,
        request: &PostV1HrContractsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrContractsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/contracts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_contracts_end(
        &self,
        request: &PostV1HrContractsEndRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrContractsEndResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/contracts/end",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_contracts_list(
        &self,
        request: &PostV1HrContractsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrContractsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/contracts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_leave_balances_set(
        &self,
        request: &PostV1HrLeaveBalancesSetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrLeaveBalancesSetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/leave-balances/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_leave_balances_list(
        &self,
        request: &PostV1HrLeaveBalancesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrLeaveBalancesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/leave-balances/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_incapacity_certificates_create(
        &self,
        request: &PostV1HrIncapacityCertificatesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrIncapacityCertificatesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/incapacity-certificates/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_incapacity_certificates_list(
        &self,
        request: &PostV1HrIncapacityCertificatesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrIncapacityCertificatesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/incapacity-certificates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_records_create(
        &self,
        request: &PostV1HrEmployeesRecordsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesRecordsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/records/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_records_update(
        &self,
        request: &PostV1HrEmployeesRecordsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesRecordsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/records/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_records_delete(
        &self,
        request: &PostV1HrEmployeesRecordsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesRecordsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/records/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_records_list(
        &self,
        request: &PostV1HrEmployeesRecordsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesRecordsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/records/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_employees_attachments_list(
        &self,
        request: &PostV1HrEmployeesAttachmentsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrEmployeesAttachmentsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/attachments/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_timesheets_generate(
        &self,
        request: &PostV1HrTimesheetsGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrTimesheetsGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/timesheets/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_timesheets_upsert(
        &self,
        request: &PostV1HrTimesheetsUpsertRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrTimesheetsUpsertResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/timesheets/upsert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_timesheets_get(
        &self,
        request: &PostV1HrTimesheetsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrTimesheetsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/timesheets/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_timesheets_list(
        &self,
        request: &PostV1HrTimesheetsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrTimesheetsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/timesheets/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_hr_timesheets_delete(
        &self,
        request: &PostV1HrTimesheetsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1HrTimesheetsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/timesheets/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
