use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PayrollClient {
    pub http_client: HttpClient,
}

impl PayrollClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn departments_create(
        &self,
        request: &DepartmentsCreatePayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<DepartmentsCreatePayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/departments/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn departments_list(
        &self,
        request: &DepartmentsListPayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<DepartmentsListPayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/departments/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn schedules_create(
        &self,
        request: &SchedulesCreatePayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<SchedulesCreatePayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/schedules/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn schedules_list(
        &self,
        request: &SchedulesListPayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<SchedulesListPayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/schedules/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn calc(
        &self,
        request: &CalcPayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<CalcPayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/calc",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn runs_create(
        &self,
        request: &RunsCreatePayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<RunsCreatePayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/runs/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn runs_get(
        &self,
        request: &RunsGetPayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<RunsGetPayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/runs/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn runs_list(
        &self,
        request: &RunsListPayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<RunsListPayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/runs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The days and hours worked, the days on the register and the average hourly earnings that some countries report per employment. The Czech monthly employer report asks for all four. They can be set while the run is a draft.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn lines_attendance(
        &self,
        request: &LinesAttendancePayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<LinesAttendancePayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/lines/attendance",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn runs_approve(
        &self,
        request: &RunsApprovePayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<RunsApprovePayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/runs/approve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn runs_reverse(
        &self,
        request: &RunsReversePayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<RunsReversePayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/runs/reverse",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn runs_cancel(
        &self,
        request: &RunsCancelPayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<RunsCancelPayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/runs/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn payments_export(
        &self,
        request: &PaymentsExportPayrollRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentsExportPayrollResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/payroll/payments/export",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
