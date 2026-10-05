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
