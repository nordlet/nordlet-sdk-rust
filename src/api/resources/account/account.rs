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

    pub async fn login_link_request(
        &self,
        request: &LoginLinkRequestAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<LoginLinkRequestAccountResponse, ApiError> {
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

    pub async fn login_link_consume(
        &self,
        request: &LoginLinkConsumeAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<LoginLinkConsumeAccountResponse, ApiError> {
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

    pub async fn logout(
        &self,
        request: &LogoutAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<LogoutAccountResponse, ApiError> {
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

    pub async fn me(
        &self,
        request: &MeAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<MeAccountResponse, ApiError> {
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

    pub async fn members_list(
        &self,
        request: &MembersListAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<MembersListAccountResponse, ApiError> {
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

    pub async fn members_set_role(
        &self,
        request: &MembersSetRoleAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<MembersSetRoleAccountResponse, ApiError> {
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

    pub async fn members_transfer_ownership(
        &self,
        request: &MembersTransferOwnershipAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<MembersTransferOwnershipAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/members/transfer-ownership",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn members_remove(
        &self,
        request: &MembersRemoveAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<MembersRemoveAccountResponse, ApiError> {
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

    pub async fn invites_create(
        &self,
        request: &InvitesCreateAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvitesCreateAccountResponse, ApiError> {
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

    pub async fn invites_list(
        &self,
        request: &InvitesListAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvitesListAccountResponse, ApiError> {
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

    pub async fn invites_revoke(
        &self,
        request: &InvitesRevokeAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvitesRevokeAccountResponse, ApiError> {
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

    pub async fn invites_get(
        &self,
        request: &InvitesGetAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvitesGetAccountResponse, ApiError> {
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

    pub async fn invites_accept(
        &self,
        request: &InvitesAcceptAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvitesAcceptAccountResponse, ApiError> {
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

    pub async fn locale_set(
        &self,
        request: &LocaleSetAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<LocaleSetAccountResponse, ApiError> {
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

    pub async fn companies_create(
        &self,
        request: &CompaniesCreateAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompaniesCreateAccountResponse, ApiError> {
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

    pub async fn companies_select(
        &self,
        request: &CompaniesSelectAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompaniesSelectAccountResponse, ApiError> {
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

    pub async fn companies_profile(
        &self,
        request: &CompaniesProfileAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompaniesProfileAccountResponse, ApiError> {
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

    pub async fn companies_update(
        &self,
        request: &CompaniesUpdateAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompaniesUpdateAccountResponse, ApiError> {
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

    pub async fn companies_archive(
        &self,
        request: &CompaniesArchiveAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompaniesArchiveAccountResponse, ApiError> {
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

    pub async fn companies_delete(
        &self,
        request: &CompaniesDeleteAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompaniesDeleteAccountResponse, ApiError> {
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

    pub async fn companies_activate(
        &self,
        request: &CompaniesActivateAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompaniesActivateAccountResponse, ApiError> {
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

    pub async fn api_keys_create(
        &self,
        request: &ApiKeysCreateAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeysCreateAccountResponse, ApiError> {
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

    pub async fn api_keys_list(
        &self,
        request: &ApiKeysListAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeysListAccountResponse, ApiError> {
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

    pub async fn api_keys_rotate(
        &self,
        request: &ApiKeysRotateAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeysRotateAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/api-keys/rotate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn api_keys_revoke(
        &self,
        request: &ApiKeysRevokeAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeysRevokeAccountResponse, ApiError> {
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

    pub async fn consent_accept(
        &self,
        request: &ConsentAcceptAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConsentAcceptAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/consent/accept",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn profile_update(
        &self,
        request: &ProfileUpdateAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProfileUpdateAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/profile/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn email_change_request(
        &self,
        request: &EmailChangeRequestAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmailChangeRequestAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/email/change-request",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn sessions_list(
        &self,
        request: &SessionsListAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionsListAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/sessions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn sessions_revoke(
        &self,
        request: &SessionsRevokeAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionsRevokeAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/sessions/revoke",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn sessions_revoke_others(
        &self,
        request: &SessionsRevokeOthersAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<SessionsRevokeOthersAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/sessions/revoke-others",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn export(
        &self,
        request: &ExportAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExportAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/export",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Removes the user: sessions, sign-in links, memberships and pending invitations are deleted at once; the email and name are replaced by an anonymous placeholder immediately and the remaining row is removed after 30 days. Refused while the user still owns or pays for a company that is not deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        request: &DeleteAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn referral_get(
        &self,
        request: &ReferralGetAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReferralGetAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/referral/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn referral_convert(
        &self,
        request: &ReferralConvertAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReferralConvertAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/referral/convert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn table_settings_get(
        &self,
        request: &TableSettingsGetAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<TableSettingsGetAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/table-settings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn table_settings_set(
        &self,
        request: &TableSettingsSetAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<TableSettingsSetAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/table-settings/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn table_settings_list(
        &self,
        request: &TableSettingsListAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<TableSettingsListAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/account/table-settings/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
