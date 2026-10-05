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

    /// Replaces the seeded chart with the chart template of the company country (the Romanian general chart for a company registered in Romania, the Lithuanian standard chart otherwise) and switches the posting defaults with it. Answers 409 when the company already uses that chart, has journal entries, holds accounts created by hand, or has settings that name an account the new chart does not have.
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

    /// The rows or codes of each return or registry deposit of the company country that are filled from account balances. Accounts fall into a row by the layout defaults for the standard chart of accounts unless mapped under Settings → Statement rows.
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
