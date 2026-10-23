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

    /// Return whether automatic agreement billing is switched on for the company.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_get(
        &self,
        request: &SettingsGetAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsGetAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/settings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Switch automatic agreement billing on or off for the company. With it on, a scheduled job runs the same billing as `agreements/agreements/billing/run` for the company once a day.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_update(
        &self,
        request: &SettingsUpdateAgreementsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsUpdateAgreementsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agreements/settings/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create an agreement type with a `code` and a `name`. Answers 409 when the company already has a type with that code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's agreement types. Filterable and sortable by `code` and `name`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create an agreement for exactly one party, a partner, an employee or a bank account, which must exist in the company (400 otherwise). Without `kind`, an employee agreement is `employment`, a bank account agreement is `bank` and a partner agreement is `other`; a `documentRef` (the `storageKey` of an uploaded file) attaches that file to the agreement. Answers 409 when the company already has an agreement with that `number`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return one agreement with its items.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change the given fields of an agreement and return it with its items. The party, `number`, `startDate`, `currency` and items are not changed by this call.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Mark an agreement as deleted. It is no longer returned, changed or billed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's agreements without their items, each with `partnerName`. Filterable by `number`, `name`, `partnerId`, `employeeId`, `bankAccountId`, `kind`, `typeId`, `status`, `startDate`, `endDate` and `value`; sortable by `number`, `startDate` and `createdAt` (default `startDate` descending).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create a draft sale invoice for the next unbilled period of an active agreement that has a partner and a `billingPeriod`. The period follows the monthly, quarterly or annual grid from `startDate`, continues after the last period already invoiced for the agreement, and is prorated by days when it starts late or is cut by `endDate`; an `autoRenew` agreement has its `endDate` moved forward by whole terms instead (`renewedEndDate`). The lines come from the agreement items, else one line of `value`, with revenue recognised ratably over the period. Answers 422 when the agreement is not active, has no partner, billing period, items or value, carries VAT while the company is not a VAT payer, is fully billed through its end date, or when the next period starts after `asOfDate` (default today). Emits `agreement.invoice_generated`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Generate the invoices due up to `asOfDate` (default today) for every active agreement of the company that has a partner and a billing period, one invoice per period and at most 60 per agreement in one run, in the same way as `agreements/agreements/generate-invoice`. An agreement that is fully billed and whose `endDate` has passed is set to `expired`. A failing agreement is listed in `errors` and does not stop the run. Each invoice emits `agreement.invoice_generated`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create an insurance policy. Answers 422 when `toDate` is before `fromDate`, 400 when `insurerPartnerId` is not a partner of the company, and 409 when the company already has a policy with that `policyNumber`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's insurance policies. Filterable by `policyNumber`, `insurerPartnerId` and `toDate`; sortable by `policyNumber` and `toDate` (default `toDate` descending).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Mark an insurance policy as deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
