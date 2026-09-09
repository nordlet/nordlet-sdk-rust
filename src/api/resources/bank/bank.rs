use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BankClient {
    pub http_client: HttpClient,
}

impl BankClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_bank_accounts_create(
        &self,
        request: &PostV1BankAccountsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankAccountsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/accounts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_accounts_list(
        &self,
        request: &PostV1BankAccountsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankAccountsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/accounts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_accounts_update(
        &self,
        request: &PostV1BankAccountsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankAccountsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/accounts/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_transactions_import(
        &self,
        request: &PostV1BankTransactionsImportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankTransactionsImportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_statements_import(
        &self,
        request: &PostV1BankStatementsImportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankStatementsImportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/statements/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_transactions_list(
        &self,
        request: &PostV1BankTransactionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankTransactionsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_transactions_match(
        &self,
        request: &PostV1BankTransactionsMatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankTransactionsMatchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/match",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_transactions_record(
        &self,
        request: &PostV1BankTransactionsRecordRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankTransactionsRecordResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/record",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_payments_export(
        &self,
        request: &PostV1BankPaymentsExportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankPaymentsExportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/payments/export",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn create_a_bank_import_template_fields_default_to_the_types_standard_field_list(
        &self,
        request: &PostV1BankImportTemplatesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankImportTemplatesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_import_templates_update(
        &self,
        request: &PostV1BankImportTemplatesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankImportTemplatesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_import_templates_delete(
        &self,
        request: &PostV1BankImportTemplatesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankImportTemplatesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_import_templates_get(
        &self,
        request: &PostV1BankImportTemplatesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankImportTemplatesGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_import_templates_list(
        &self,
        request: &PostV1BankImportTemplatesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankImportTemplatesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/import-templates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_match_rules_create(
        &self,
        request: &PostV1BankMatchRulesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMatchRulesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/match-rules/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_match_rules_update(
        &self,
        request: &PostV1BankMatchRulesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMatchRulesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/match-rules/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_match_rules_delete(
        &self,
        request: &PostV1BankMatchRulesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMatchRulesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/match-rules/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_match_rules_list(
        &self,
        request: &PostV1BankMatchRulesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMatchRulesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/match-rules/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_mandates_create(
        &self,
        request: &PostV1BankMandatesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMandatesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_mandates_update(
        &self,
        request: &PostV1BankMandatesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMandatesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_mandates_cancel(
        &self,
        request: &PostV1BankMandatesCancelRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMandatesCancelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_mandates_get(
        &self,
        request: &PostV1BankMandatesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMandatesGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_mandates_list(
        &self,
        request: &PostV1BankMandatesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankMandatesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/mandates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_direct_debits_export(
        &self,
        request: &PostV1BankDirectDebitsExportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankDirectDebitsExportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/direct-debits/export",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_transactions_suggest_matches(
        &self,
        request: &PostV1BankTransactionsSuggestMatchesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankTransactionsSuggestMatchesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/suggest-matches",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_settlements_import(
        &self,
        request: &PostV1BankSettlementsImportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankSettlementsImportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_settlements_list(
        &self,
        request: &PostV1BankSettlementsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankSettlementsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_settlements_get(
        &self,
        request: &PostV1BankSettlementsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankSettlementsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_settlements_match(
        &self,
        request: &PostV1BankSettlementsMatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankSettlementsMatchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/match",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Attach the incoming bank-statement line that carries this payout to the settlement batch.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_bank_settlements_link(
        &self,
        request: &PostV1BankSettlementsLinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankSettlementsLinkResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/link",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Detach the bank-statement line from the settlement batch and return the line to unmatched.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_bank_settlements_unlink(
        &self,
        request: &PostV1BankSettlementsUnlinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankSettlementsUnlinkResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/unlink",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_settlements_post(
        &self,
        request: &PostV1BankSettlementsPostRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankSettlementsPostResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/post",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn list_the_psd2_banks_asps_ps_available_to_connect(
        &self,
        request: &PostV1BankFeedsBanksListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsBanksListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/banks/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn begin_bank_authorization_redirect_the_user_to_the_returned_url(
        &self,
        request: &PostV1BankFeedsConnectionsStartRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsConnectionsStartResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/start",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn exchange_the_redirect_code_for_a_session_and_store_the_bank_accounts_it_exposes(
        &self,
        request: &PostV1BankFeedsConnectionsCompleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsConnectionsCompleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/complete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_feeds_connections_get(
        &self,
        request: &PostV1BankFeedsConnectionsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsConnectionsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_bank_feeds_connections_list(
        &self,
        request: &PostV1BankFeedsConnectionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsConnectionsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn revoke_the_consent_at_the_bank_and_drop_the_stored_connection(
        &self,
        request: &PostV1BankFeedsConnectionsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsConnectionsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/connections/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn point_a_bank_feed_account_at_a_ledger_bank_account_so_its_transactions_can_be_synced(
        &self,
        request: &PostV1BankFeedsAccountsLinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsAccountsLinkResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/accounts/link",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn choose_the_import_template_applied_on_sync_and_how_often_the_account_is_synced_automatically(
        &self,
        request: &PostV1BankFeedsAccountsConfigureRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsAccountsConfigureResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/accounts/configure",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn pull_new_transactions_from_the_bank_into_the_ledger_emits_bank_feed_synced(
        &self,
        request: &PostV1BankFeedsSyncRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1BankFeedsSyncResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/feeds/sync",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
