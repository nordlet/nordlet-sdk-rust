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
}
