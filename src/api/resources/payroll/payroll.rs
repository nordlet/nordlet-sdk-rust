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

    pub async fn post_v1_payroll_departments_create(
        &self,
        request: &PostV1PayrollDepartmentsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollDepartmentsCreateResponse, ApiError> {
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

    pub async fn post_v1_payroll_departments_list(
        &self,
        request: &PostV1PayrollDepartmentsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollDepartmentsListResponse, ApiError> {
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

    pub async fn post_v1_payroll_schedules_create(
        &self,
        request: &PostV1PayrollSchedulesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollSchedulesCreateResponse, ApiError> {
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

    pub async fn post_v1_payroll_schedules_list(
        &self,
        request: &PostV1PayrollSchedulesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollSchedulesListResponse, ApiError> {
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

    pub async fn post_v1_payroll_calc(
        &self,
        request: &PostV1PayrollCalcRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollCalcResponse, ApiError> {
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

    pub async fn post_v1_payroll_runs_create(
        &self,
        request: &PostV1PayrollRunsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollRunsCreateResponse, ApiError> {
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

    pub async fn post_v1_payroll_runs_get(
        &self,
        request: &PostV1PayrollRunsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollRunsGetResponse, ApiError> {
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

    pub async fn post_v1_payroll_runs_list(
        &self,
        request: &PostV1PayrollRunsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollRunsListResponse, ApiError> {
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

    pub async fn post_v1_payroll_runs_approve(
        &self,
        request: &PostV1PayrollRunsApproveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollRunsApproveResponse, ApiError> {
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

    pub async fn post_v1_payroll_runs_cancel(
        &self,
        request: &PostV1PayrollRunsCancelRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollRunsCancelResponse, ApiError> {
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

    pub async fn post_v1_payroll_payments_export(
        &self,
        request: &PostV1PayrollPaymentsExportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PayrollPaymentsExportResponse, ApiError> {
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
