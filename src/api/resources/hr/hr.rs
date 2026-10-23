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

    /// Creates a job position with an optional code and name translations, for use in `hr/contracts/create`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Changes the code, name or translations of a position; a null `translations` removes them.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the company's positions; filterable and sortable by `name` and `code`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Creates an employee record. `payrollOptions` holds the country-specific settings the payroll engine reads for the person (see `hr/employees/fields`); `applyAllowance` defaults to true and `pensionAccumulation` to false.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Changes the given fields of an employee; a null clears the fields that allow it (code, personal code, birth date, contact details, address, IBAN, social insurance number and start, hire date, notes). `status` and `terminationDate` are set here too.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Reads one employee.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the employees that are not deleted; filterable by `firstName`, `lastName`, `code`, `status` and `email`, sortable by `lastName`, `code` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Marks the employee deleted; the record stays and no longer appears in lists or payroll runs. `hr/employees/anonymize` also blanks the personal data.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Creates an employment contract for an employee. A `permanent` contract takes no `endDate` and a `fixed_term` one requires it; the number comes from `contractNo` or, with `agreementId`, from the employee's agreement; `workHours` defaults by `salaryType`. Answers 422 when the end date is before the start, and 409 when the employee already has a contract overlapping the period (end it with `hr/contracts/end` first) or the contract number already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Ends a contract on `endDate` with an optional reason. Answers 422 when the end date is before the contract start.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists employment contracts; filterable by `employeeId`, `agreementId`, `status` and `contractNo`, sortable by `startDate` and `contractNo`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Creates or replaces the leave entitlement of an employee for a year; `usedDays` is kept unless given (0 on a new row) and `remainingDays` is the entitled days less the used ones.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists leave balances, narrowed to an employee and/or a year when given, oldest year first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Records a sick-leave certificate of an employee. Answers 422 when `toDate` is before `fromDate`. `payroll/runs/create` warns when a certificate covers working days of a month that has no timesheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists incapacity certificates; filterable by `employeeId` and `number`, sortable by `fromDate` and `number`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Adds the company's daily per diem rate for a destination country from `validFrom`; `hr/business-trips/create` takes the latest rate valid on the trip's start date. Answers 409 when a rate for that country already starts on that date.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn per_diem_rates_create(
        &self,
        request: &PerDiemRatesCreateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<PerDiemRatesCreateHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/per-diem-rates/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists per diem rates; filterable by `countryCode`, sortable by `countryCode` and `validFrom`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn per_diem_rates_list(
        &self,
        request: &PerDiemRatesListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<PerDiemRatesListHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/per-diem-rates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a per diem rate.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn per_diem_rates_delete(
        &self,
        request: &PerDiemRatesDeleteHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<PerDiemRatesDeleteHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/per-diem-rates/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Stores the trip as a draft with `days` counted as calendar days, both ends included, and `perDiemAmount` as the days times the company's rate for the destination valid on the start date (`hr/per-diem-rates/create`). Answers 422 when the trip ends before it starts or no rate covers the destination on that date, and 409 when the employee already has a trip overlapping the dates.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn business_trips_create(
        &self,
        request: &BusinessTripsCreateHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessTripsCreateHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/business-trips/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reads one business trip.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn business_trips_get(
        &self,
        request: &BusinessTripsGetHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessTripsGetHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/business-trips/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists business trips; filterable by `employeeId`, `status` and `payrollRunId`, sortable by `startDate`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn business_trips_list(
        &self,
        request: &BusinessTripsListHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessTripsListHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/business-trips/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Sets the trip's status to `approved`. `payroll/runs/create` without explicit lines adds the per diem of every approved trip ending in the run month as an untaxed addition and sets the trip's `payrollRunId`; reversing or cancelling the run releases the trip again. Answers 409 when the trip is not a draft.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn business_trips_approve(
        &self,
        request: &BusinessTripsApproveHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessTripsApproveHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/business-trips/approve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a draft trip; answers 409 when the trip is not a draft.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn business_trips_delete(
        &self,
        request: &BusinessTripsDeleteHrRequest,
        options: Option<RequestOptions>,
    ) -> Result<BusinessTripsDeleteHrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/hr/business-trips/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds an education, qualification, certificate or training record to an employee, optionally linked to one of the company's uploaded files (`fileId`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Changes the given fields of an employee record; `issuedAt`, `validUntil` and `fileId` can be cleared with null.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Deletes an employee record.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists employee records; filterable by `employeeId`, `type` and `title`, sortable by `createdAt` and `issuedAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the files uploaded against an employee, newest first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Builds the timesheet of the month for every employee with a contract in it, or for one employee with `employeeId`, replacing any existing timesheet: the Monday to Friday days within the contract get the contract's daily hours (its weekly hours over five), and for a Lithuanian company the public holidays are `holiday` days with 0 hours. Answers 422 when no contract covers the month.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Creates or replaces the timesheet of an employee for a month from the given days; `workedDays` and `workedHours` count the `work` and `business_trip` days with hours. Answers 422 for a day outside the month, a day given twice or more than 24 hours in a day. `payroll/runs/create` reads the timesheet for the paid days, the days worked and the business-trip days of the month.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Reads the timesheet of an employee for a month.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the timesheets of a month, ordered by employee name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Deletes a timesheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
