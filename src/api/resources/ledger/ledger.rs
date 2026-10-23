use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LedgerClient {
    pub http_client: HttpClient,
}

impl LedgerClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the company's chart of accounts. Filterable by `code`, `name`, `type` and `isPostable`; sortable by `code` and `name`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn accounts_list(
        &self,
        request: &AccountsListLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsListLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/accounts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds an account to the company's chart of accounts. `parentId` must name an account of the same company. Answers 409 when an account with the same `code` already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn accounts_create(
        &self,
        request: &AccountsCreateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsCreateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/accounts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the name, parent or postability of an account; the code and type cannot be changed. An account cannot be its own parent.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn accounts_update(
        &self,
        request: &AccountsUpdateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsUpdateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/accounts/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds the accounts of the company's chart template that the chart does not have yet (an account whose code already exists is not replaced) and sets the parent links the template defines. Returns the number of accounts in the chart afterwards.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn accounts_apply_template(
        &self,
        request: &AccountsApplyTemplateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsApplyTemplateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/accounts/apply-template",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replaces the seeded chart with the chart template of the company country (the Romanian general chart for a company registered in Romania, the Estonian e-arveldaja chart for a company registered in Estonia, the Lithuanian standard chart otherwise) and switches the posting defaults with it. Answers 409 when the company already uses that chart, has journal entries, holds accounts created by hand, or has settings that name an account the new chart does not have.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn accounts_switch_chart(
        &self,
        request: &AccountsSwitchChartLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsSwitchChartLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/accounts/switch-chart",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the company's accounting periods with their `open` or `locked` status. Filterable by `year`, `month` and `status`; sortable by `year` and `month`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn periods_list(
        &self,
        request: &PeriodsListLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PeriodsListLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/periods/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Locks the month so that postings dated in it are refused with 409 (`ledger/journal/transactions/create`, `cash/orders/create` and `cash/expense-reports/create` among them). Before locking a month that is not locked yet, posts the pending ratable revenue recognition tranches scheduled up to the month end and emits `revenue_recognition.posted` when there were any; when the month is the company's fiscal year end and no year-end close is posted for that year, posts the year-end close that moves the income and expense balances of the fiscal year to the retained earnings account (posting rule `closing.retainedEarnings`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn periods_lock(
        &self,
        request: &PeriodsLockLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PeriodsLockLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/periods/lock",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Opens the month for posting again. When the month is the company's fiscal year end and a year-end close is posted for it, posts a reversal of that close dated the same day.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn periods_unlock(
        &self,
        request: &PeriodsUnlockLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PeriodsUnlockLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/periods/unlock",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists journal transactions with the partner name. Filterable by `date`, `status`, `documentType`, `partnerId` and `documentId`; sortable by `date` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn journal_transactions_list(
        &self,
        request: &JournalTransactionsListLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<JournalTransactionsListLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/journal/transactions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a cost center, optionally inside a cost center group. Answers 409 when the `code` is already used.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cost_centers_create(
        &self,
        request: &CostCentersCreateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCentersCreateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/cost-centers/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the name, group or active flag of a cost center. An inactive cost center is refused on new manual journal entries (`ledger/journal/transactions/create` answers 422).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cost_centers_update(
        &self,
        request: &CostCentersUpdateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCentersUpdateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/cost-centers/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists cost centers with their group name. Filterable by `code`, `name`, `isActive` and `groupId`; sortable by `code`, `name` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cost_centers_list(
        &self,
        request: &CostCentersListLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCentersListLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/cost-centers/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a cost center group. Answers 409 when the `code` is already used.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cost_center_groups_create(
        &self,
        request: &CostCenterGroupsCreateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCenterGroupsCreateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/cost-center-groups/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the code or name of a cost center group. Answers 409 when the new code is already used.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cost_center_groups_update(
        &self,
        request: &CostCenterGroupsUpdateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCenterGroupsUpdateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/cost-center-groups/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a cost center group. Answers 409 while cost centers are still assigned to it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cost_center_groups_delete(
        &self,
        request: &CostCenterGroupsDeleteLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCenterGroupsDeleteLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/cost-center-groups/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists cost center groups. Filterable and sortable by `code` and `name`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cost_center_groups_list(
        &self,
        request: &CostCenterGroupsListLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CostCenterGroupsListLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/cost-center-groups/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists every posting rule with its default account code on the company's chart template, the account code in effect and whether the company has overridden it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn posting_rules_list(
        &self,
        request: &PostingRulesListLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostingRulesListLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/posting-rules/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Overrides the account code of one or more posting rules; an `accountCode` of null removes the override and the chart default applies again. Every code given must be a postable account of the company's chart, else 422.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn posting_rules_update(
        &self,
        request: &PostingRulesUpdateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostingRulesUpdateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/posting-rules/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds an owner of the company. `equityAccountCode` defaults to the share capital account of the company's chart; a code that is not in the chart of accounts answers 422.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn owners_create(
        &self,
        request: &OwnersCreateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<OwnersCreateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/owners/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the fields of an owner. An `equityAccountCode` that is not in the chart of accounts answers 422.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn owners_update(
        &self,
        request: &OwnersUpdateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<OwnersUpdateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/owners/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an owner.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn owners_delete(
        &self,
        request: &OwnersDeleteLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<OwnersDeleteLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/owners/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the company's owners. Filterable by `name` and `code`; sortable by `name` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn owners_list(
        &self,
        request: &OwnersListLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<OwnersListLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/owners/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a journal transaction with its entries, ordered by account code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn journal_transactions_get(
        &self,
        request: &JournalTransactionsGetLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<JournalTransactionsGetLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/journal/transactions/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Posts a manual journal transaction (`documentType` `manual`, status `posted`) in the company's base currency. Every `accountCode` must be a postable account of the chart, cost centers must exist and be active, projects must exist, and the debits must equal the credits. Answers 422 for an amount with more than two decimals, for a `currency` other than the base currency and for an inactive cost center, and 409 when the accounting period of `date` is locked.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn journal_transactions_create(
        &self,
        request: &JournalTransactionsCreateLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<JournalTransactionsCreateLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/journal/transactions/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The rows or codes of each return or registry deposit of the company country that are filled from account balances. Accounts fall into a row by the layout defaults for the standard chart of accounts unless mapped under Account → Statement rows.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn statement_rows_schemes(
        &self,
        request: &StatementRowsSchemesLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatementRowsSchemesLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/statement-rows/schemes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Every postable account of the company with the row of `scheme` it falls into, by a mapping (`ledger/statement-rows/set`) or by the layout default, and the amount it carries there: the balance up to `toDate` on a balance-sheet row, the movement from `fromDate` to `toDate` on an income-statement row. `toDate` defaults to today and `fromDate` to 1 January of that year. `rows` holds the row totals and `unmapped` the accounts with a balance but no row.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn statement_rows_list(
        &self,
        request: &StatementRowsListLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatementRowsListLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/statement-rows/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// A mapping on a code prefix covers every account whose code starts with it; the longest matching prefix wins. An empty rowCode removes the mapping so the layout default applies again.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn statement_rows_set(
        &self,
        request: &StatementRowsSetLedgerRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatementRowsSetLedgerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ledger/statement-rows/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
