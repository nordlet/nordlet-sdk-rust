use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ReportsClient {
    pub http_client: HttpClient,
}

impl ReportsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_reports_trial_balance(
        &self,
        request: &PostV1ReportsTrialBalanceRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsTrialBalanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/trial-balance",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_size_category(
        &self,
        request: &PostV1ReportsSizeCategoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsSizeCategoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/size-category",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_financial_statements(
        &self,
        request: &PostV1ReportsFinancialStatementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsFinancialStatementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/financial-statements",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_general_journal(
        &self,
        request: &PostV1ReportsGeneralJournalRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsGeneralJournalResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/general-journal",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_gl_detail(
        &self,
        request: &PostV1ReportsGlDetailRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsGlDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/gl-detail",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_partner_balances(
        &self,
        request: &PostV1ReportsPartnerBalancesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsPartnerBalancesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/partner-balances",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_debt_aging(
        &self,
        request: &PostV1ReportsDebtAgingRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsDebtAgingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/debt-aging",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_monthly_summary(
        &self,
        request: &PostV1ReportsMonthlySummaryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsMonthlySummaryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/monthly-summary",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_stock_balance(
        &self,
        request: &PostV1ReportsStockBalanceRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsStockBalanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/stock-balance",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_stock_movement(
        &self,
        request: &PostV1ReportsStockMovementRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsStockMovementResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/stock-movement",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_vat_summary(
        &self,
        request: &PostV1ReportsVatSummaryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsVatSummaryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/vat-summary",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_cash_flow(
        &self,
        request: &PostV1ReportsCashFlowRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsCashFlowResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/cash-flow",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_stock_aging(
        &self,
        request: &PostV1ReportsStockAgingRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsStockAgingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/stock-aging",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_stock_shortage(
        &self,
        request: &PostV1ReportsStockShortageRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsStockShortageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/stock-shortage",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_eu_purchases(
        &self,
        request: &PostV1ReportsEuPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsEuPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/eu-purchases",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_vat_detail(
        &self,
        request: &PostV1ReportsVatDetailRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsVatDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/vat-detail",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_pos_sales(
        &self,
        request: &PostV1ReportsPosSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsPosSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/pos-sales",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_online_sales(
        &self,
        request: &PostV1ReportsOnlineSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsOnlineSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/online-sales",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_oss(
        &self,
        request: &PostV1ReportsOssRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsOssResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/oss",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_advance_reconciliation(
        &self,
        request: &PostV1ReportsAdvanceReconciliationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsAdvanceReconciliationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/advance-reconciliation",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_write_off_acts(
        &self,
        request: &PostV1ReportsWriteOffActsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsWriteOffActsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/write-off-acts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_cost_centers(
        &self,
        request: &PostV1ReportsCostCentersRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsCostCentersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/cost-centers",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_cost_center_activity(
        &self,
        request: &PostV1ReportsCostCenterActivityRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsCostCenterActivityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/cost-center-activity",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_cost_center_items(
        &self,
        request: &PostV1ReportsCostCenterItemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsCostCenterItemsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/cost-center-items",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_jobs_create(
        &self,
        request: &PostV1ReportsJobsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsJobsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/jobs/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_jobs_get(
        &self,
        request: &PostV1ReportsJobsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsJobsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/jobs/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reports_jobs_list(
        &self,
        request: &PostV1ReportsJobsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReportsJobsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/jobs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
