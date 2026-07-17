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

    pub async fn post_v1_partners_addresses_create(
        &self,
        request: &PostV1PartnersAddressesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersAddressesCreateResponse, ApiError> {
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

    pub async fn post_v1_partners_addresses_update(
        &self,
        request: &PostV1PartnersAddressesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersAddressesUpdateResponse, ApiError> {
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

    pub async fn post_v1_partners_addresses_delete(
        &self,
        request: &PostV1PartnersAddressesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersAddressesDeleteResponse, ApiError> {
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

    pub async fn post_v1_partners_addresses_list(
        &self,
        request: &PostV1PartnersAddressesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersAddressesListResponse, ApiError> {
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

    pub async fn post_v1_partners_contacts_create(
        &self,
        request: &PostV1PartnersContactsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersContactsCreateResponse, ApiError> {
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

    pub async fn post_v1_partners_contacts_update(
        &self,
        request: &PostV1PartnersContactsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersContactsUpdateResponse, ApiError> {
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

    pub async fn post_v1_partners_contacts_delete(
        &self,
        request: &PostV1PartnersContactsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersContactsDeleteResponse, ApiError> {
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

    pub async fn post_v1_partners_contacts_list(
        &self,
        request: &PostV1PartnersContactsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersContactsListResponse, ApiError> {
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

    pub async fn post_v1_partners_bank_accounts_create(
        &self,
        request: &PostV1PartnersBankAccountsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersBankAccountsCreateResponse, ApiError> {
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

    pub async fn post_v1_partners_bank_accounts_update(
        &self,
        request: &PostV1PartnersBankAccountsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersBankAccountsUpdateResponse, ApiError> {
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

    pub async fn post_v1_partners_bank_accounts_delete(
        &self,
        request: &PostV1PartnersBankAccountsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersBankAccountsDeleteResponse, ApiError> {
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

    pub async fn post_v1_partners_bank_accounts_list(
        &self,
        request: &PostV1PartnersBankAccountsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersBankAccountsListResponse, ApiError> {
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

    pub async fn post_v1_partners_validate_vat(
        &self,
        request: &PostV1PartnersValidateVatRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersValidateVatResponse, ApiError> {
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

    pub async fn post_v1_partners_create(
        &self,
        request: &PostV1PartnersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersCreateResponse, ApiError> {
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

    pub async fn post_v1_partners_find_or_create(
        &self,
        request: &PostV1PartnersFindOrCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersFindOrCreateResponse, ApiError> {
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

    pub async fn post_v1_partners_get(
        &self,
        request: &PostV1PartnersGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersGetResponse, ApiError> {
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

    pub async fn post_v1_partners_update(
        &self,
        request: &PostV1PartnersUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersUpdateResponse, ApiError> {
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

    pub async fn post_v1_partners_delete(
        &self,
        request: &PostV1PartnersDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersDeleteResponse, ApiError> {
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

    pub async fn post_v1_partners_list(
        &self,
        request: &PostV1PartnersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersListResponse, ApiError> {
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

    pub async fn post_v1_partners_groups_create(
        &self,
        request: &PostV1PartnersGroupsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersGroupsCreateResponse, ApiError> {
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

    pub async fn post_v1_partners_groups_update(
        &self,
        request: &PostV1PartnersGroupsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersGroupsUpdateResponse, ApiError> {
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

    pub async fn post_v1_partners_groups_delete(
        &self,
        request: &PostV1PartnersGroupsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersGroupsDeleteResponse, ApiError> {
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

    pub async fn post_v1_partners_groups_list(
        &self,
        request: &PostV1PartnersGroupsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersGroupsListResponse, ApiError> {
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

    pub async fn post_v1_partners_statuses_create(
        &self,
        request: &PostV1PartnersStatusesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersStatusesCreateResponse, ApiError> {
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

    pub async fn post_v1_partners_statuses_update(
        &self,
        request: &PostV1PartnersStatusesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersStatusesUpdateResponse, ApiError> {
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

    pub async fn post_v1_partners_statuses_delete(
        &self,
        request: &PostV1PartnersStatusesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersStatusesDeleteResponse, ApiError> {
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

    pub async fn post_v1_partners_statuses_list(
        &self,
        request: &PostV1PartnersStatusesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersStatusesListResponse, ApiError> {
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

    pub async fn post_v1_partners_inquiries_create(
        &self,
        request: &PostV1PartnersInquiriesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersInquiriesCreateResponse, ApiError> {
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

    pub async fn post_v1_partners_inquiries_update(
        &self,
        request: &PostV1PartnersInquiriesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersInquiriesUpdateResponse, ApiError> {
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

    pub async fn post_v1_partners_inquiries_get(
        &self,
        request: &PostV1PartnersInquiriesGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersInquiriesGetResponse, ApiError> {
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

    pub async fn post_v1_partners_inquiries_list(
        &self,
        request: &PostV1PartnersInquiriesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersInquiriesListResponse, ApiError> {
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

    pub async fn post_v1_partners_credit_check(
        &self,
        request: &PostV1PartnersCreditCheckRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1PartnersCreditCheckResponse, ApiError> {
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
