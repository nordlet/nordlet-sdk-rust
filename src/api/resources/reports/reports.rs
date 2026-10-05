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

    pub async fn trial_balance(
        &self,
        request: &TrialBalanceReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TrialBalanceReportsResponse, ApiError> {
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

    pub async fn size_category(
        &self,
        request: &SizeCategoryReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SizeCategoryReportsResponse, ApiError> {
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

    pub async fn financial_statements(
        &self,
        request: &FinancialStatementsReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<FinancialStatementsReportsResponse, ApiError> {
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

    pub async fn general_journal(
        &self,
        request: &GeneralJournalReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GeneralJournalReportsResponse, ApiError> {
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

    pub async fn gl_detail(
        &self,
        request: &GlDetailReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GlDetailReportsResponse, ApiError> {
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

    pub async fn partner_balances(
        &self,
        request: &PartnerBalancesReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PartnerBalancesReportsResponse, ApiError> {
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

    pub async fn debt_aging(
        &self,
        request: &DebtAgingReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DebtAgingReportsResponse, ApiError> {
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

    pub async fn monthly_summary(
        &self,
        request: &MonthlySummaryReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MonthlySummaryReportsResponse, ApiError> {
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

    pub async fn stock_balance(
        &self,
        request: &StockBalanceReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockBalanceReportsResponse, ApiError> {
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

    pub async fn stock_movement(
        &self,
        request: &StockMovementReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockMovementReportsResponse, ApiError> {
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

    pub async fn vat_summary(
        &self,
        request: &VatSummaryReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatSummaryReportsResponse, ApiError> {
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

    pub async fn cash_flow(
        &self,
        request: &CashFlowReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CashFlowReportsResponse, ApiError> {
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

    pub async fn stock_aging(
        &self,
        request: &StockAgingReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockAgingReportsResponse, ApiError> {
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

    pub async fn stock_shortage(
        &self,
        request: &StockShortageReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockShortageReportsResponse, ApiError> {
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

    /// Export the ledger of one financial year as an SIE file (the Swedish standard accounting interchange format, specification 4B). The file carries the chart of accounts, the opening and closing balance of every balance sheet account and the turnover of every result account for the year and the year before it, and, when asked for, every posted voucher of the year with its lines. Cost centres travel as dimension 1 and projects as dimension 6. Services that build a Swedish annual report read this file.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sie(
        &self,
        request: &SieReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SieReportsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/sie",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Export the posted ledger of a period as a DATEV Buchungsstapel file (DATEV format, category 21, version 700). Every transaction becomes one or more bookings of an amount between an account and a contra account; a transaction with more than two lines is split into pairs whose totals match it. The file is semicolon separated and written in the Windows-1252 character set DATEV expects.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn datev(
        &self,
        request: &DatevReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatevReportsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/datev",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Export the posted ledger of a period as a French FEC file (fichier des écritures comptables, order of 29 July 2013). One line per journal entry line, with the eighteen fields the order names, in their order, after a header line. Tab separated, UTF-8, comma as the decimal separator.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn fec(
        &self,
        request: &FecReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<FecReportsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reports/fec",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn eu_purchases(
        &self,
        request: &EuPurchasesReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuPurchasesReportsResponse, ApiError> {
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

    pub async fn vat_detail(
        &self,
        request: &VatDetailReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatDetailReportsResponse, ApiError> {
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

    pub async fn pos_sales(
        &self,
        request: &PosSalesReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PosSalesReportsResponse, ApiError> {
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

    pub async fn online_sales(
        &self,
        request: &OnlineSalesReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<OnlineSalesReportsResponse, ApiError> {
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

    pub async fn oss(
        &self,
        request: &OssReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<OssReportsResponse, ApiError> {
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

    pub async fn advance_reconciliation(
        &self,
        request: &AdvanceReconciliationReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdvanceReconciliationReportsResponse, ApiError> {
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

    pub async fn write_off_acts(
        &self,
        request: &WriteOffActsReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<WriteOffActsReportsResponse, ApiError> {
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

    pub async fn cost_centers(
        &self,
        request: &CostCentersReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCentersReportsResponse, ApiError> {
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

    pub async fn cost_center_activity(
        &self,
        request: &CostCenterActivityReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCenterActivityReportsResponse, ApiError> {
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

    pub async fn cost_center_items(
        &self,
        request: &CostCenterItemsReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCenterItemsReportsResponse, ApiError> {
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

    pub async fn jobs_create(
        &self,
        request: &JobsCreateReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<JobsCreateReportsResponse, ApiError> {
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

    pub async fn jobs_get(
        &self,
        request: &JobsGetReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<JobsGetReportsResponse, ApiError> {
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

    pub async fn jobs_list(
        &self,
        request: &JobsListReportsRequest,
        options: Option<RequestOptions>,
    ) -> Result<JobsListReportsResponse, ApiError> {
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
