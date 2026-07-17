use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AccountClient {
    pub http_client: HttpClient,
}

impl AccountClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_account_login_link_request(
        &self,
        request: &PostV1AccountLoginLinkRequestRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountLoginLinkRequestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/login-link/request",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_login_link_consume(
        &self,
        request: &PostV1AccountLoginLinkConsumeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountLoginLinkConsumeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/login-link/consume",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_logout(
        &self,
        request: &PostV1AccountLogoutRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountLogoutResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/logout",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_me(
        &self,
        request: &PostV1AccountMeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountMeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/me",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_members_list(
        &self,
        request: &PostV1AccountMembersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountMembersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/members/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_members_set_role(
        &self,
        request: &PostV1AccountMembersSetRoleRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountMembersSetRoleResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/members/set-role",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_members_remove(
        &self,
        request: &PostV1AccountMembersRemoveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountMembersRemoveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/members/remove",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_invites_create(
        &self,
        request: &PostV1AccountInvitesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountInvitesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/invites/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_invites_list(
        &self,
        request: &PostV1AccountInvitesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountInvitesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/invites/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_invites_revoke(
        &self,
        request: &PostV1AccountInvitesRevokeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountInvitesRevokeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/invites/revoke",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_invites_get(
        &self,
        request: &PostV1AccountInvitesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountInvitesGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/invites/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_invites_accept(
        &self,
        request: &PostV1AccountInvitesAcceptRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountInvitesAcceptResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/invites/accept",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_locale_set(
        &self,
        request: &PostV1AccountLocaleSetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountLocaleSetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/locale/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_companies_create(
        &self,
        request: &PostV1AccountCompaniesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountCompaniesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/companies/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_companies_select(
        &self,
        request: &PostV1AccountCompaniesSelectRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountCompaniesSelectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/companies/select",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_companies_profile(
        &self,
        request: &PostV1AccountCompaniesProfileRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountCompaniesProfileResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/companies/profile",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_companies_update(
        &self,
        request: &PostV1AccountCompaniesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountCompaniesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/companies/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_companies_archive(
        &self,
        request: &PostV1AccountCompaniesArchiveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountCompaniesArchiveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/companies/archive",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_companies_delete(
        &self,
        request: &PostV1AccountCompaniesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountCompaniesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/companies/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_companies_activate(
        &self,
        request: &PostV1AccountCompaniesActivateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountCompaniesActivateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/companies/activate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_api_keys_create(
        &self,
        request: &PostV1AccountApiKeysCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountApiKeysCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/api-keys/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_api_keys_list(
        &self,
        request: &PostV1AccountApiKeysListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountApiKeysListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/api-keys/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_account_api_keys_revoke(
        &self,
        request: &PostV1AccountApiKeysRevokeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AccountApiKeysRevokeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/api-keys/revoke",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
