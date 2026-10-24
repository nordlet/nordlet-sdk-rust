use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PartnersClient {
    pub http_client: HttpClient,
}

impl PartnersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Adds an address of type billing, shipping, registered or other to the partner in `partnerId`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn addresses_create(
        &self,
        request: &AddressesCreatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddressesCreatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/addresses/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the given fields of a partner address.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn addresses_update(
        &self,
        request: &AddressesUpdatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddressesUpdatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/addresses/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a partner address.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn addresses_delete(
        &self,
        request: &AddressesDeletePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddressesDeletePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/addresses/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists partner addresses; filterable by `partnerId`, `type` and `city`, sortable by `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn addresses_list(
        &self,
        request: &AddressesListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddressesListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/addresses/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds a contact person to the partner in `partnerId`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn contacts_create(
        &self,
        request: &ContactsCreatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<ContactsCreatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/contacts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the given fields of a partner contact.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn contacts_update(
        &self,
        request: &ContactsUpdatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<ContactsUpdatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/contacts/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a partner contact.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn contacts_delete(
        &self,
        request: &ContactsDeletePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<ContactsDeletePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/contacts/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists partner contacts; filterable by `partnerId`, `name` and `email`, sortable by `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn contacts_list(
        &self,
        request: &ContactsListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<ContactsListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/contacts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds a bank account to the partner in `partnerId`. The IBAN is stored without spaces in upper case; an IBAN the partner already has is refused with 409. With `isDefault` true the partner's other accounts lose their default mark.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn bank_accounts_create(
        &self,
        request: &BankAccountsCreatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<BankAccountsCreatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/bank-accounts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the given fields of a partner bank account. A new IBAN is stored without spaces in upper case and is refused with 409 when the partner already has it; when the account is the default after the change, the partner's other accounts lose their default mark.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn bank_accounts_update(
        &self,
        request: &BankAccountsUpdatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<BankAccountsUpdatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/bank-accounts/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a partner bank account.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn bank_accounts_delete(
        &self,
        request: &BankAccountsDeletePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<BankAccountsDeletePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/bank-accounts/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists partner bank accounts; filterable by `partnerId` and `iban`, sortable by `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn bank_accounts_list(
        &self,
        request: &BankAccountsListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<BankAccountsListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/bank-accounts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the files attached to a partner, newest first. `partners/create` attaches one with `documentRef`, and `leads/convert` moves the lead's files here.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn files_list(
        &self,
        request: &FilesListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<FilesListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/files/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Runs the overdue selection of the daily debt-reminder job for this company as of the company's current date and returns the overdue reminders it would send, without sending or recording anything. A partner is included when it has `autoDebtReminder`, is active, has an e-mail address, got no overdue reminder in the last `repeatDays` days (`partners/debt-reminders/settings/get`) and has issued invoices or advance invoices past their due date with an unpaid remainder; the interest per invoice is the remainder times the partner's `lateInterestPercent` times the days late over 365, and the totals are per currency. Sent reminders are listed by `partners/debt-reminders/list`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn debt_reminders_preview(
        &self,
        request: &DebtRemindersPreviewPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<DebtRemindersPreviewPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/debt-reminders/preview",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the debt reminders the daily job has sent, one row per partner and currency with the ids of the invoices it covered and its `kind` (`before_due` or `overdue`); filterable by `partnerId`, sortable by `sentAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn debt_reminders_list(
        &self,
        request: &DebtRemindersListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<DebtRemindersListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/debt-reminders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the debt-reminder schedule of the company: `daysBefore`, the days before the due date on which the first reminder is sent (null when no reminder is sent before the due date), and `repeatDays`, the days between overdue reminders to a partner.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn debt_reminders_settings_get(
        &self,
        request: &DebtRemindersSettingsGetPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<DebtRemindersSettingsGetPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/debt-reminders/settings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the given fields of the company's debt-reminder schedule. `daysBefore` (1 to 90, or null to send nothing before the due date) makes the daily job send one "payment due soon" reminder per invoice that many days before its due date; `repeatDays` (1 to 90) is the number of days between overdue reminders to a partner. Only partners with `autoDebtReminder` get reminders. A body without fields is refused with 400.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn debt_reminders_settings_update(
        &self,
        request: &DebtRemindersSettingsUpdatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<DebtRemindersSettingsUpdatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/debt-reminders/settings/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the next reminder the daily debt-reminder job would send for each unpaid issued invoice or advance invoice of the partners that get reminders (`autoDebtReminder`, active, with an e-mail address), as of the company's current date and sorted by `date`. A `before_due` reminder is sent once per invoice, `daysBefore` days before its due date or on the first daily run after that while the due date is still ahead; `overdue` reminders start the day after the due date and repeat every `repeatDays` days per partner, counting only overdue reminders. Nothing is sent or recorded.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn debt_reminders_upcoming(
        &self,
        request: &DebtRemindersUpcomingPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<DebtRemindersUpcomingPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/debt-reminders/upcoming",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Checks a VAT number against VIES, either a free `vatCode` or the VAT code of the partner in `partnerId` (exactly one of the two); a code without an EU country prefix and 2 to 12 alphanumeric characters is refused with 422, as is a partner without a VAT code, and a VIES outage answers 503. With a partner the outcome is written to it (`vatValid`, `vatValidatedAt`): an invalid number or a VIES name that shares no word with the partner's name opens or refreshes the partner's VAT review (`vat_review.opened`), a VIES failure opens one with reason `service_error`, and a valid matching number resolves the open review as `revalidated` (`vat_review.resolved`). Reviews are listed and closed with `partners/vat-reviews/list` and `partners/vat-reviews/resolve`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn validate_vat(
        &self,
        request: &ValidateVatPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<ValidateVatPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/validate-vat",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the VAT reviews opened by VIES checks of partners' VAT codes, with reason `invalid`, `name_mismatch` or `service_error`; filterable by `partnerId`, `status` and `reason`, sortable by `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn vat_reviews_list(
        &self,
        request: &VatReviewsListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatReviewsListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/vat-reviews/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Closes an open VAT review as `confirmed_valid`, `confirmed_invalid` or `dismissed`, with an optional note. The first two also set the partner's `vatValid` and stamp `vatValidatedAt` when the partner still carries the reviewed VAT code; `dismissed` leaves the partner unchanged. A review that is already resolved answers 409; emits `vat_review.resolved`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn vat_reviews_resolve(
        &self,
        request: &VatReviewsResolvePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatReviewsResolvePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/vat-reviews/resolve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a partner. `groupId`, `statusId` and `priceListId` must belong to the company; without `legalCountryClass` the class is derived from the address country (`lt`, `eu` or `non_eu`); `documentRef` attaches an uploaded file that is not yet linked to a record; a `code` already used by a live partner of the company is refused with 409. A `vatCode` is checked against VIES at once and the result stored on the partner: an invalid number, a VIES name that shares no word with the partner's name, or a VIES failure opens a VAT review (`vat_review.opened`), while a code that is not an EU VAT number is stored unchecked (see `partners/validate-vat`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the live partner matched by `code`, then `vatCode`, then `email` (at least one is required), or creates one from the body with the same validation and VIES check as `partners/create`; `created` says which happened. A creation that loses a race on the partner code returns the partner the other call created.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn find_or_create(
        &self,
        request: &FindOrCreatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<FindOrCreatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/find-or-create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one partner; a deleted partner answers 404.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        request: &GetPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the given fields of a partner; the clearable fields accept null. A new address country re-derives `legalCountryClass` unless the class is given, and a `code` already used by another live partner of the company is refused with 409. A changed `vatCode` resets the VIES result and is checked at once as in `partners/create`; a removed code, or one that is not an EU VAT number, resolves the partner's open VAT review as `superseded` (`vat_review.resolved`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        request: &UpdatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Marks a partner as deleted; it no longer appears in lists or lookups.
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
        request: &DeletePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeletePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Moves every row that references the duplicate `sourceId` through a foreign key to partners (looked up in the database catalogue, within the company) to `targetId`, then marks the source partner as deleted; `moved` lists the tables, columns and row counts. Merging a partner into itself is 422, and when both partners hold the same unique entry (for example the same IBAN or item supplier row) the merge is refused with 409 until it is removed from one of them. When the kept partner has an open VAT review, the duplicate's open review is resolved as `superseded` (`vat_review.resolved`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn merge(
        &self,
        request: &MergePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<MergePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/merge",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Removes birth date, self-employment certificate number, email, phone, address, notes, contacts, addresses and bank accounts, then hides the partner. The name, code and VAT number stay because issued invoices must keep identifying the counterparty for the statutory retention period.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn anonymize(
        &self,
        request: &AnonymizePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnonymizePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/anonymize",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists live partners; filterable by `name`, `code`, `vatCode`, `email`, `type`, `isCustomer`, `isSupplier`, `isActive`, `legalCountryClass`, `isEmployee`, `groupId`, `statusId` and `nextCallDate`, sortable by `name`, `code`, `createdAt`, `nextCallDate` and `rating`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a partner group; a `code` already used by a group of the company is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn groups_create(
        &self,
        request: &GroupsCreatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsCreatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/groups/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the code or name of a partner group; a code already used by another group of the company is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn groups_update(
        &self,
        request: &GroupsUpdatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsUpdatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/groups/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a partner group; a group still assigned to a live partner is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn groups_delete(
        &self,
        request: &GroupsDeletePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsDeletePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/groups/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the company's partner groups ordered by code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn groups_list(
        &self,
        request: &GroupsListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/groups/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a partner status with a `sortOrder`; a `code` already used by a status of the company is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn statuses_create(
        &self,
        request: &StatusesCreatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatusesCreatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/statuses/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the code, name or sort order of a partner status; a code already used by another status of the company is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn statuses_update(
        &self,
        request: &StatusesUpdatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatusesUpdatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/statuses/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a partner status; a status still assigned to a live partner is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn statuses_delete(
        &self,
        request: &StatusesDeletePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatusesDeletePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/statuses/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the company's partner statuses ordered by sort order, then code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn statuses_list(
        &self,
        request: &StatusesListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<StatusesListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/statuses/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Records an inquiry in status `new`, from a partner (`partnerId`) or from a contact given by `contactName` or `contactEmail`; without any of the three it is refused with 422. `assignedUserId` must be a member of the company. Emits `partner_inquiry.created`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn inquiries_create(
        &self,
        request: &InquiriesCreatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<InquiriesCreatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/inquiries/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the given fields of an inquiry; `partnerId` and `assignedUserId` accept null. Setting `status` to `closed` stamps `closedAt`, and moving a closed inquiry to another status clears it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn inquiries_update(
        &self,
        request: &InquiriesUpdatePartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<InquiriesUpdatePartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/inquiries/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one inquiry with the name of its partner.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn inquiries_get(
        &self,
        request: &InquiriesGetPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<InquiriesGetPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/inquiries/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists inquiries with their partner names; filterable by `status`, `channel`, `partnerId`, `assignedUserId` and `subject`, sortable by `createdAt` and `updatedAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn inquiries_list(
        &self,
        request: &InquiriesListPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<InquiriesListPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/inquiries/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Compares a partner's open receivables plus an optional `additionalAmount` with its `creditLimit`. The open receivables are the unpaid remainder of the partner's issued invoices, advance invoices and credit notes (credit notes negative), converted to the company's base currency at each invoice's `fxRate`. Without a credit limit `available` is null and `exceeded` is false.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn credit_check(
        &self,
        request: &CreditCheckPartnersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreditCheckPartnersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/partners/credit-check",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
