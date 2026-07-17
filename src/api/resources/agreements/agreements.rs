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

    pub async fn post_v1_agreements_types_create(
        &self,
        request: &PostV1AgreementsTypesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsTypesCreateResponse, ApiError> {
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

    pub async fn post_v1_agreements_types_list(
        &self,
        request: &PostV1AgreementsTypesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsTypesListResponse, ApiError> {
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

    pub async fn post_v1_agreements_agreements_create(
        &self,
        request: &PostV1AgreementsAgreementsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsAgreementsCreateResponse, ApiError> {
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

    pub async fn post_v1_agreements_agreements_get(
        &self,
        request: &PostV1AgreementsAgreementsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsAgreementsGetResponse, ApiError> {
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

    pub async fn post_v1_agreements_agreements_update(
        &self,
        request: &PostV1AgreementsAgreementsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsAgreementsUpdateResponse, ApiError> {
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

    pub async fn post_v1_agreements_agreements_delete(
        &self,
        request: &PostV1AgreementsAgreementsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsAgreementsDeleteResponse, ApiError> {
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

    pub async fn post_v1_agreements_agreements_list(
        &self,
        request: &PostV1AgreementsAgreementsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsAgreementsListResponse, ApiError> {
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

    pub async fn post_v1_agreements_insurance_policies_create(
        &self,
        request: &PostV1AgreementsInsurancePoliciesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsInsurancePoliciesCreateResponse, ApiError> {
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

    pub async fn post_v1_agreements_insurance_policies_list(
        &self,
        request: &PostV1AgreementsInsurancePoliciesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsInsurancePoliciesListResponse, ApiError> {
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

    pub async fn post_v1_agreements_insurance_policies_delete(
        &self,
        request: &PostV1AgreementsInsurancePoliciesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AgreementsInsurancePoliciesDeleteResponse, ApiError> {
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
