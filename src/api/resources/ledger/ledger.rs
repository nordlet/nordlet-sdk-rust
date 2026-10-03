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

    pub async fn post_v1_ledger_accounts_list(
        &self,
        request: &PostV1LedgerAccountsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerAccountsListResponse, ApiError> {
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

    pub async fn post_v1_ledger_accounts_create(
        &self,
        request: &PostV1LedgerAccountsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerAccountsCreateResponse, ApiError> {
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

    pub async fn post_v1_ledger_accounts_update(
        &self,
        request: &PostV1LedgerAccountsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerAccountsUpdateResponse, ApiError> {
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

    pub async fn post_v1_ledger_accounts_apply_template(
        &self,
        request: &PostV1LedgerAccountsApplyTemplateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerAccountsApplyTemplateResponse, ApiError> {
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
    pub async fn move_a_company_that_has_posted_nothing_yet_to_the_chart_of_accounts_of_its_country(
        &self,
        request: &PostV1LedgerAccountsSwitchChartRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerAccountsSwitchChartResponse, ApiError> {
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

    pub async fn post_v1_ledger_periods_list(
        &self,
        request: &PostV1LedgerPeriodsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerPeriodsListResponse, ApiError> {
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

    pub async fn post_v1_ledger_periods_lock(
        &self,
        request: &PostV1LedgerPeriodsLockRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerPeriodsLockResponse, ApiError> {
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

    pub async fn post_v1_ledger_periods_unlock(
        &self,
        request: &PostV1LedgerPeriodsUnlockRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerPeriodsUnlockResponse, ApiError> {
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

    pub async fn post_v1_ledger_journal_transactions_list(
        &self,
        request: &PostV1LedgerJournalTransactionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerJournalTransactionsListResponse, ApiError> {
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

    pub async fn post_v1_ledger_cost_centers_create(
        &self,
        request: &PostV1LedgerCostCentersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerCostCentersCreateResponse, ApiError> {
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

    pub async fn post_v1_ledger_cost_centers_update(
        &self,
        request: &PostV1LedgerCostCentersUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerCostCentersUpdateResponse, ApiError> {
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

    pub async fn post_v1_ledger_cost_centers_list(
        &self,
        request: &PostV1LedgerCostCentersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerCostCentersListResponse, ApiError> {
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

    pub async fn post_v1_ledger_cost_center_groups_create(
        &self,
        request: &PostV1LedgerCostCenterGroupsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerCostCenterGroupsCreateResponse, ApiError> {
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

    pub async fn post_v1_ledger_cost_center_groups_update(
        &self,
        request: &PostV1LedgerCostCenterGroupsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerCostCenterGroupsUpdateResponse, ApiError> {
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

    pub async fn post_v1_ledger_cost_center_groups_delete(
        &self,
        request: &PostV1LedgerCostCenterGroupsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerCostCenterGroupsDeleteResponse, ApiError> {
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

    pub async fn post_v1_ledger_cost_center_groups_list(
        &self,
        request: &PostV1LedgerCostCenterGroupsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerCostCenterGroupsListResponse, ApiError> {
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

    pub async fn post_v1_ledger_posting_rules_list(
        &self,
        request: &PostV1LedgerPostingRulesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerPostingRulesListResponse, ApiError> {
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

    pub async fn post_v1_ledger_posting_rules_update(
        &self,
        request: &PostV1LedgerPostingRulesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerPostingRulesUpdateResponse, ApiError> {
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

    pub async fn post_v1_ledger_owners_create(
        &self,
        request: &PostV1LedgerOwnersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerOwnersCreateResponse, ApiError> {
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

    pub async fn post_v1_ledger_owners_update(
        &self,
        request: &PostV1LedgerOwnersUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerOwnersUpdateResponse, ApiError> {
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

    pub async fn post_v1_ledger_owners_delete(
        &self,
        request: &PostV1LedgerOwnersDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerOwnersDeleteResponse, ApiError> {
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

    pub async fn post_v1_ledger_owners_list(
        &self,
        request: &PostV1LedgerOwnersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerOwnersListResponse, ApiError> {
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

    pub async fn post_v1_ledger_journal_transactions_get(
        &self,
        request: &PostV1LedgerJournalTransactionsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerJournalTransactionsGetResponse, ApiError> {
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

    pub async fn post_v1_ledger_journal_transactions_create(
        &self,
        request: &PostV1LedgerJournalTransactionsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerJournalTransactionsCreateResponse, ApiError> {
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
    pub async fn national_statement_layouts_available_to_the_company(
        &self,
        request: &PostV1LedgerStatementRowsSchemesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerStatementRowsSchemesResponse, ApiError> {
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

    pub async fn accounts_placed_on_the_rows_of_a_statement_layout_with_the_row_totals_of_a_period(
        &self,
        request: &PostV1LedgerStatementRowsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerStatementRowsListResponse, ApiError> {
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
    pub async fn map_an_account_or_an_account_code_prefix_to_a_row_of_a_statement_layout(
        &self,
        request: &PostV1LedgerStatementRowsSetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1LedgerStatementRowsSetResponse, ApiError> {
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

    /// Directors, board members, the company secretary, representatives and liquidators, with their personal identifier, appointment and resignation dates and whether they sign the annual accounts. Annual returns and registry deposits are built from this register.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn officers_of_the_company(
        &self,
        request: &PostV1OfficersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1OfficersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/officers/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn record_an_officer_of_the_company(
        &self,
        request: &PostV1OfficersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1OfficersCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/officers/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn change_a_recorded_officer(
        &self,
        request: &PostV1OfficersUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1OfficersUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/officers/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn remove_a_recorded_officer(
        &self,
        request: &PostV1OfficersDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1OfficersDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/officers/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
