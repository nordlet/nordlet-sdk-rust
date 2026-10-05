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

    pub async fn accounts_create(
        &self,
        request: &AccountsCreateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsCreateBankResponse, ApiError> {
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

    pub async fn accounts_list(
        &self,
        request: &AccountsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsListBankResponse, ApiError> {
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

    pub async fn accounts_update(
        &self,
        request: &AccountsUpdateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountsUpdateBankResponse, ApiError> {
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

    pub async fn transactions_import(
        &self,
        request: &TransactionsImportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsImportBankResponse, ApiError> {
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

    pub async fn statements_import(
        &self,
        request: &StatementsImportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatementsImportBankResponse, ApiError> {
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

    pub async fn transactions_list(
        &self,
        request: &TransactionsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsListBankResponse, ApiError> {
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

    pub async fn transactions_match(
        &self,
        request: &TransactionsMatchBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsMatchBankResponse, ApiError> {
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

    /// Undo a match. A payment matched to an invoice, or a line posted by an import template, gets a reversing journal transaction dated date (default: today) and the invoice paid amount and payment status are restored; a line linked to a payment-provider settlement is only unlinked. The line returns to status new.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn transactions_unmatch(
        &self,
        request: &TransactionsUnmatchBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsUnmatchBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/transactions/unmatch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn transactions_record(
        &self,
        request: &TransactionsRecordBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsRecordBankResponse, ApiError> {
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

    pub async fn payments_export(
        &self,
        request: &PaymentsExportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentsExportBankResponse, ApiError> {
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

    pub async fn import_templates_create(
        &self,
        request: &ImportTemplatesCreateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesCreateBankResponse, ApiError> {
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

    pub async fn import_templates_update(
        &self,
        request: &ImportTemplatesUpdateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesUpdateBankResponse, ApiError> {
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

    pub async fn import_templates_delete(
        &self,
        request: &ImportTemplatesDeleteBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesDeleteBankResponse, ApiError> {
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

    pub async fn import_templates_get(
        &self,
        request: &ImportTemplatesGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesGetBankResponse, ApiError> {
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

    pub async fn import_templates_list(
        &self,
        request: &ImportTemplatesListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportTemplatesListBankResponse, ApiError> {
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

    pub async fn match_rules_create(
        &self,
        request: &MatchRulesCreateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchRulesCreateBankResponse, ApiError> {
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

    pub async fn match_rules_update(
        &self,
        request: &MatchRulesUpdateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchRulesUpdateBankResponse, ApiError> {
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

    pub async fn match_rules_delete(
        &self,
        request: &MatchRulesDeleteBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchRulesDeleteBankResponse, ApiError> {
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

    pub async fn match_rules_list(
        &self,
        request: &MatchRulesListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchRulesListBankResponse, ApiError> {
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

    pub async fn mandates_create(
        &self,
        request: &MandatesCreateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesCreateBankResponse, ApiError> {
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

    pub async fn mandates_update(
        &self,
        request: &MandatesUpdateBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesUpdateBankResponse, ApiError> {
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

    pub async fn mandates_cancel(
        &self,
        request: &MandatesCancelBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesCancelBankResponse, ApiError> {
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

    pub async fn mandates_get(
        &self,
        request: &MandatesGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesGetBankResponse, ApiError> {
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

    pub async fn mandates_list(
        &self,
        request: &MandatesListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<MandatesListBankResponse, ApiError> {
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

    pub async fn direct_debits_export(
        &self,
        request: &DirectDebitsExportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<DirectDebitsExportBankResponse, ApiError> {
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

    pub async fn transactions_suggest_matches(
        &self,
        request: &TransactionsSuggestMatchesBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransactionsSuggestMatchesBankResponse, ApiError> {
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

    pub async fn settlements_import(
        &self,
        request: &SettlementsImportBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsImportBankResponse, ApiError> {
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

    pub async fn settlements_list(
        &self,
        request: &SettlementsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsListBankResponse, ApiError> {
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

    pub async fn settlements_get(
        &self,
        request: &SettlementsGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsGetBankResponse, ApiError> {
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

    pub async fn settlements_match(
        &self,
        request: &SettlementsMatchBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsMatchBankResponse, ApiError> {
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

    /// A line with its own rate or amount is split with that value when the batch is posted. A line without one falls back to the commissionPercent given to the posting call, and without that the amount goes to the suspense account. Send both fields as null to clear the line back to the fallback.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settlements_commission(
        &self,
        request: &SettlementsCommissionBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsCommissionBankResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/bank/settlements/commission",
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
    pub async fn settlements_link(
        &self,
        request: &SettlementsLinkBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsLinkBankResponse, ApiError> {
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
    pub async fn settlements_unlink(
        &self,
        request: &SettlementsUnlinkBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsUnlinkBankResponse, ApiError> {
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

    pub async fn settlements_post(
        &self,
        request: &SettlementsPostBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettlementsPostBankResponse, ApiError> {
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

    pub async fn feeds_banks_list(
        &self,
        request: &FeedsBanksListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsBanksListBankResponse, ApiError> {
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

    pub async fn feeds_connections_start(
        &self,
        request: &FeedsConnectionsStartBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsStartBankResponse, ApiError> {
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

    pub async fn feeds_connections_complete(
        &self,
        request: &FeedsConnectionsCompleteBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsCompleteBankResponse, ApiError> {
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

    pub async fn feeds_connections_get(
        &self,
        request: &FeedsConnectionsGetBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsGetBankResponse, ApiError> {
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

    pub async fn feeds_connections_list(
        &self,
        request: &FeedsConnectionsListBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsListBankResponse, ApiError> {
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

    pub async fn feeds_connections_delete(
        &self,
        request: &FeedsConnectionsDeleteBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsConnectionsDeleteBankResponse, ApiError> {
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

    pub async fn feeds_accounts_link(
        &self,
        request: &FeedsAccountsLinkBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsAccountsLinkBankResponse, ApiError> {
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

    pub async fn feeds_accounts_configure(
        &self,
        request: &FeedsAccountsConfigureBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsAccountsConfigureBankResponse, ApiError> {
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

    pub async fn feeds_sync(
        &self,
        request: &FeedsSyncBankRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeedsSyncBankResponse, ApiError> {
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
