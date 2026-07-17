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
}
