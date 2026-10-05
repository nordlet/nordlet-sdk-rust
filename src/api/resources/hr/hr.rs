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

    pub async fn positions_create(
        &self,
        request: &PositionsCreateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<PositionsCreateHrResponse, ApiError> {
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

    pub async fn positions_update(
        &self,
        request: &PositionsUpdateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<PositionsUpdateHrResponse, ApiError> {
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

    pub async fn positions_list(
        &self,
        request: &PositionsListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<PositionsListHrResponse, ApiError> {
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

    pub async fn employees_create(
        &self,
        request: &EmployeesCreateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesCreateHrResponse, ApiError> {
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

    pub async fn employees_update(
        &self,
        request: &EmployeesUpdateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesUpdateHrResponse, ApiError> {
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

    pub async fn employees_get(
        &self,
        request: &EmployeesGetHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesGetHrResponse, ApiError> {
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

    /// Attributes a filing of the company country needs about a person that the shared employee record does not carry, such as the sex and place of birth an Italian income certificate asks for. Their values are kept in the payrollOptions of the employee.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn employees_fields(
        &self,
        request: &EmployeesFieldsHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesFieldsHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/fields",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn employees_list(
        &self,
        request: &EmployeesListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesListHrResponse, ApiError> {
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

    pub async fn employees_delete(
        &self,
        request: &EmployeesDeleteHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesDeleteHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replaces the name with a placeholder and removes personal code, birth date, contact details, address, bank account, social-insurance number, notes and sick-leave reasons. Payroll and contract rows stay linked to the record for the statutory retention period.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn employees_anonymize(
        &self,
        request: &EmployeesAnonymizeHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesAnonymizeHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/employees/anonymize",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn contracts_create(
        &self,
        request: &ContractsCreateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<ContractsCreateHrResponse, ApiError> {
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

    pub async fn contracts_end(
        &self,
        request: &ContractsEndHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<ContractsEndHrResponse, ApiError> {
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

    pub async fn contracts_list(
        &self,
        request: &ContractsListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<ContractsListHrResponse, ApiError> {
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

    pub async fn leave_balances_set(
        &self,
        request: &LeaveBalancesSetHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<LeaveBalancesSetHrResponse, ApiError> {
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

    pub async fn leave_balances_list(
        &self,
        request: &LeaveBalancesListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<LeaveBalancesListHrResponse, ApiError> {
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

    pub async fn incapacity_certificates_create(
        &self,
        request: &IncapacityCertificatesCreateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<IncapacityCertificatesCreateHrResponse, ApiError> {
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

    pub async fn incapacity_certificates_list(
        &self,
        request: &IncapacityCertificatesListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<IncapacityCertificatesListHrResponse, ApiError> {
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

    pub async fn employees_records_create(
        &self,
        request: &EmployeesRecordsCreateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesRecordsCreateHrResponse, ApiError> {
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

    pub async fn employees_records_update(
        &self,
        request: &EmployeesRecordsUpdateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesRecordsUpdateHrResponse, ApiError> {
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

    pub async fn employees_records_delete(
        &self,
        request: &EmployeesRecordsDeleteHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesRecordsDeleteHrResponse, ApiError> {
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

    pub async fn employees_records_list(
        &self,
        request: &EmployeesRecordsListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesRecordsListHrResponse, ApiError> {
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

    pub async fn employees_attachments_list(
        &self,
        request: &EmployeesAttachmentsListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmployeesAttachmentsListHrResponse, ApiError> {
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

    pub async fn timesheets_generate(
        &self,
        request: &TimesheetsGenerateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimesheetsGenerateHrResponse, ApiError> {
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

    pub async fn timesheets_upsert(
        &self,
        request: &TimesheetsUpsertHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimesheetsUpsertHrResponse, ApiError> {
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

    pub async fn timesheets_get(
        &self,
        request: &TimesheetsGetHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimesheetsGetHrResponse, ApiError> {
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

    pub async fn timesheets_list(
        &self,
        request: &TimesheetsListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimesheetsListHrResponse, ApiError> {
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

    pub async fn timesheets_delete(
        &self,
        request: &TimesheetsDeleteHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimesheetsDeleteHrResponse, ApiError> {
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
