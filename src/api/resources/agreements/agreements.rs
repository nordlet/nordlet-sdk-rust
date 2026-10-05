use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AgreementsClient {
    pub http_client: HttpClient,
}

impl AgreementsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn types_create(
        &self,
        request: &TypesCreateAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesCreateAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/types/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn types_list(
        &self,
        request: &TypesListAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesListAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/types/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn agreements_create(
        &self,
        request: &AgreementsCreateAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgreementsCreateAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/agreements/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn agreements_get(
        &self,
        request: &AgreementsGetAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgreementsGetAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/agreements/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn agreements_update(
        &self,
        request: &AgreementsUpdateAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgreementsUpdateAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/agreements/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn agreements_delete(
        &self,
        request: &AgreementsDeleteAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgreementsDeleteAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/agreements/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn agreements_list(
        &self,
        request: &AgreementsListAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgreementsListAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/agreements/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn agreements_generate_invoice(
        &self,
        request: &AgreementsGenerateInvoiceAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgreementsGenerateInvoiceAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/agreements/generate-invoice",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn agreements_billing_run(
        &self,
        request: &AgreementsBillingRunAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgreementsBillingRunAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/agreements/billing/run",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn insurance_policies_create(
        &self,
        request: &InsurancePoliciesCreateAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<InsurancePoliciesCreateAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/insurance-policies/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn insurance_policies_list(
        &self,
        request: &InsurancePoliciesListAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<InsurancePoliciesListAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/insurance-policies/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn insurance_policies_delete(
        &self,
        request: &InsurancePoliciesDeleteAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<InsurancePoliciesDeleteAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/insurance-policies/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
