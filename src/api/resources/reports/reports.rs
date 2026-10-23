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

    /// Trial balance of the posted ledger for a period: for every account with an opening balance or movement, the balance before `fromDate`, the debit and credit turnover between `fromDate` and `toDate`, and the closing balance, with debit and credit totals. Template account names follow the `Accept-Language` header.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Classify the company's size for a calendar year as micro, small, medium or large from total assets at the year end, the net turnover of the year and the average number of employees under employment contracts, against the limits returned in `thresholds`; a category applies when at least two of its three limits are met. This is the default category of `reports/financial-statements`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Balance sheet at `toDate` and profit and loss for the period from the posted ledger, year-end closing entries excluded, laid out by size category: `category` when given, else the one `reports/size-category` derives for the period. Micro gets the short balance sheet and profit and loss; the other categories add the balance sheet and profit and loss detail; medium and large also get the changes in equity and a cash flow summary. Line groupings follow the company's chart template.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Posted journal transactions of a period in date order, each with its lines (account, debit, credit), paged with `page` and `pageSize` and the total count. Template account names follow the `Accept-Language` header.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Ledger card of one account for a period: the opening balance before `fromDate`, every posted line in date order with a running balance, and the closing balance.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Open balance per partner: receivables from issued sale invoices and payables from registered purchase invoices that are not fully paid, credit notes reducing the balance, converted to the base currency at the document rate, with totals.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Ageing of the open receivables or payables (`side`) at `asOf` (default the company's current date) per partner: not yet due, 1 to 30, 31 to 60, 61 to 90 and over 90 days past the due date. Counts documents dated on or before `asOf` and payments made up to that date.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// One row per calendar month for the last `months` months ending with the current one: the receivable and payable balances at the end of the month and the revenue, expenses and net result posted in it, year-end closing entries excluded.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Quantity and value on hand per item and warehouse at `asOf`, summed from the stock movements dated on or before it; `warehouseId` limits the report to one warehouse. Rows with zero quantity and zero value are left out.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Stock movement per item for a period: opening quantity and value before `fromDate`, quantities and values received and issued between `fromDate` and `toDate`, and the closing figures; `warehouseId` and `itemId` narrow the report.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// VAT of issued sale invoices (by issue date) or registered purchase invoices (by registration date) in a period, summed per VAT rate: net, VAT and gross in the base currency and the number of documents, credit notes counting negative. The rows of `reports/vat-detail` grouped by rate.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Cash flow statement of a period from the posted transactions that touch a cash account: the inflow and outflow of every counterpart account, grouped into operating, investing and financing by the account prefixes of the company's chart template, with opening and closing cash and `balanced` telling whether opening cash plus the net change equals closing cash. Template account names follow the `Accept-Language` header.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Age of the stock on hand at `asOf` per item and warehouse: the remaining quantity and value of each FIFO receipt layer, bucketed by days since the receipt (0 to 30, 31 to 60, 61 to 90, over 90), with totals; `warehouseId` limits the report to one warehouse. When movements after `asOf` exist, the layers are rebuilt by replaying the movements up to that date.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Items whose reserved quantity exceeds the quantity on hand, per warehouse: on hand, reserved and the shortage. Only items with a reservation are checked; `warehouseId` limits the report to one warehouse.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Registered purchase invoices of a period (by registration date) from partners in another EU country, summed per partner country and VAT rate: net and VAT in the base currency and the number of documents, credit notes counting negative. The partner's country is the country code of its address, else the first two characters of its VAT number.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// VAT of issued sale invoices (by issue date) or registered purchase invoices (by registration date) in a period, one row per document and VAT rate: net, VAT and gross in the base currency, with totals; credit notes count negative. `reports/vat-summary` groups these rows by rate.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// POS reports of a period (by report date): per report the net, VAT, gross, cash and card amounts and the cost of goods sold, the net and VAT per VAT rate across the reports, and totals.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Sales orders created in a period, grouped by channel and currency: the number of orders, how many are fulfilled, cancelled or still open (new or reserved), and the net and gross of the orders that are not cancelled, computed from their lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Issued sale invoices and credit notes of a period (by issue date) to partners in another EU country, summed per partner country and VAT rate: net and VAT in the base currency and the number of documents. The partner's country is the country code of its address, else the first two characters of its VAT number.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Cash advances per employee for a period: the balance before `fromDate`, the advances paid out and returned through cash orders in the period, the amount settled by expense reports dated in the period, and the closing balance. A cash order counts as an advance when it names an employee and its counter account is the `cash.advances` posting rule.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Stock issued by write-offs and stock takes in a period: one row per outgoing movement with item, warehouse, quantity, cost and notes, and the total cost; `warehouseId` limits the report to one warehouse.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Income, expenses and result per cost centre for a period, from the posted journal lines assigned to a cost centre.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Posted journal lines of one cost centre for a period, summed per account: debit, credit and net. Template account names follow the `Accept-Language` header.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Net purchases per cost centre and item for a period, from the lines of registered purchase invoices (by registration date) that carry a cost centre, credit notes counting negative; `costCenterId` limits the report to one cost centre. A line without an item is reported under its description.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Queue a report to be built in the background in one or more formats (`json`, `xlsx`, `pdf`) and stored as files; the job is returned at once with status `queued`. `reportType` is one of trial-balance, partner-balances, debt-aging, stock-balance, vat-summary, vat-detail, eu-purchases, oss, pos-sales, online-sales, advance-reconciliation, write-off-acts, cost-centers, cost-center-activity, cost-center-items and cash-flow, and `params` is the body of that report route; an unknown type or invalid params are refused with 422. Follow the job with `reports/jobs/get`, download its outputs with `files/get`, and listen for `report.completed` or `report.failed`; a report that does not finish within ten minutes fails.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return one report job with its status (`queued`, `running`, `completed` or `failed`), the error of a failed job and the output files of a completed one.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's report jobs, newest first; filterable by `status` and `reportType`, sortable by `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
