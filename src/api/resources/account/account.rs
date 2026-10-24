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

    /// Create a sign-in link for the e-mail address and send it; the link is valid for 15 minutes and is consumed with `account/login-link/consume`. When an unused link was requested for the same address within the last minute, the call answers `sent` without sending another. `locale` sets the language of the e-mail and of the link; `app` set to `invoice` makes the link open the invoice app instead of the Nordlet app; `acceptTerms`, `acceptDpa` and `referralCode` are stored with the link and used when the address has no account yet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Exchange a sign-in link token for a session; an unknown, used or expired token is refused with 401. When no account has the address, one is created (`isNewUser`), which is refused with 422 unless the link was requested with `acceptTerms` and `acceptDpa`; a `referralCode` on the link records who referred the new user. A link from `account/email/change-request` changes the signed-in user's address instead, refused with 409 when another account has it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete the session the call was made with.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return the signed-in user, their active company and role in it, the companies they belong to and their consent state. `billing` is the billing account that pays for the active company (its payer, or the user when there is none or no company), with `isPayer` saying whether that is the user; `referralPoints` is the user's current points balance.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the members of the active company with their role.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change the role of a member of the active company; owners and admins may call it. Refused with 422 for the caller's own role and with 403 for the owner's, which only `account/members/transfer-ownership` changes.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Make another member the owner of the active company; the caller must be its owner and becomes an admin. Refused with 422 when `userId` is the caller. With `movePayer` the company's billing also moves to the new owner, which only the member currently paying for it may do (403).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Remove a member from the active company; owners and admins may call it. Their sessions lose this company as the active one and the API keys they created for it are revoked. Refused with 422 for the caller themself and with 403 for the owner.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create an invitation to the active company for an e-mail address with the given role and send it by e-mail in `locale`; owners and admins may call it. The invitation is valid for 7 days and replaces an earlier pending one for the same address. Refused with 409 when the address already belongs to a member.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the pending invitations of the active company; `expired` marks those past their expiry.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete a pending invitation of the active company.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Read a pending invitation by its token: the invited address, role and company name, whether it has expired and whether an account with that address already exists (`userExists`), in which case `account/invites/accept` must be called signed in as that account.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Accept an invitation and return a session with the invited company active. When the address has no account, one is created with `name` and `locale`, which is refused with 422 unless `acceptTerms` and `acceptDpa` are true; when it has one, the call must be made signed in as that account (401 otherwise). An expired invitation is refused with 422.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Set the user's language: with an active company it is stored on their membership of that company (`scope` = `membership`), otherwise on the user (`scope` = `user`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create a company with the caller as its owner and payer, seed its chart of accounts from the template of `countryCode`, and make it the active company of the session. `timeZone` defaults to the country's, `isVatPayer` to true and `defaultInvoiceCurrency` to the base currency. Refused with 422 when `baseCurrency` is not a supported currency.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Make one of the caller's companies the active company of the session. Refused with 403 when the caller is not a member and with 422 when the company is archived or deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return the profile of the active company.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change the profile of the active company; owners and admins may call it. Only the fields sent are changed, and the nullable ones can be sent as null to clear them. `logo` stores a base64-encoded image as a file of the company and sets `logoFileId`. The country, base currency and sandbox flag are fixed at creation.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Set the company named by `companyId` to `archived`; the caller must be its owner. Every session that had it active loses it. Refused with 409 unless the company is active; `account/companies/activate` reverses it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Set the company named by `companyId` to `deleted`; the caller must be its owner, and every session that had it active loses it. A sandbox company is purged at once; any other is purged by the scheduled purge after `purgeAfter` (10 days) and can be restored until then with `account/companies/activate`. Refused with 409 when it is already deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Set an archived or deleted company named by `companyId` back to `active`; the caller must be its owner. Refused with 409 when it is already active.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create an API key for the active company and return the key itself once; owners, admins and developers may call it. `scopes` defaults to the caller's own scopes and may not exceed them (403); without `expiresInDays` the key does not expire.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the API keys of the active company, including expired and revoked ones.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create a new key with the name and scopes of the key `id` and return it once; the old key keeps working for `overlapHours` and then expires, and its `replacedByKeyId` points to the new one. `expiresInDays` applies to the new key. Refused with 409 when the old key is revoked.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Revoke an API key of the active company at once.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Record the signed-in user's acceptance of the current Terms of Service and Data Processing Agreement versions and return the consent state.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Set the signed-in user's name; null or a blank string clears it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Send a confirmation link to `newEmail`; the address changes when that link is consumed with `account/login-link/consume`. When an unused link was sent to the address within the last minute, the call answers `sent` without sending another. Refused with 422 when it is already the user's address and with 409 when another account has it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the signed-in user's sessions that have not expired; `current` marks the one the call was made with.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete one of the signed-in user's sessions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete every session of the signed-in user except the one the call was made with, and return how many were deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return the user's profile, consent state, memberships, unexpired sessions, billing account, credit transactions and the audit entries they made in their companies (the most recent 5000 of each of the last two).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return the user's referral code (allocated on first call) and sign-up link, their points balance, the number of accounts they referred and the last 50 points entries. `rates.perEur` is the points the referrer earns per euro a referred user tops up, `rates.pointCents` the credit one point converts to with `account/referral/convert`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Convert `points` of the user's referral points into credit on their billing account at `rates.pointCents` per point, booked as an `adjustment` credit transaction; a billing account past its trial is set to active when the new balance allows. Refused with 422 when the user has fewer points than requested.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return the signed-in user's saved column list and page size for the table `tableKey`; both are null when nothing is saved.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Save the signed-in user's column list and page size for the table `tableKey`; only the fields sent are changed, and null clears one.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the signed-in user's saved table settings.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
