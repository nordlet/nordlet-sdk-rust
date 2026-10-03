use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DeclarationsClient {
    pub http_client: HttpClient,
}

impl DeclarationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_declarations_lt_intrastat_compute(
        &self,
        request: &PostV1DeclarationsLtIntrastatComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIntrastatComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/intrastat/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_ivaz_generate(
        &self,
        request: &PostV1DeclarationsLtIvazGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIvazGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/ivaz/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_intrastat_obligation(
        &self,
        request: &PostV1DeclarationsLtIntrastatObligationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIntrastatObligationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/intrastat/obligation",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_isaf_generate(
        &self,
        request: &PostV1DeclarationsLtIsafGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIsafGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/isaf/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_fr0600_compute(
        &self,
        request: &PostV1DeclarationsLtFr0600ComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtFr0600ComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/fr0600/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_gpm313_compute(
        &self,
        request: &PostV1DeclarationsLtGpm313ComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtGpm313ComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/gpm313/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_sam_compute(
        &self,
        request: &PostV1DeclarationsLtSamComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtSamComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/sam/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_sd_generate(
        &self,
        request: &PostV1DeclarationsLtSdGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtSdGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/sd/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_saft_generate(
        &self,
        request: &PostV1DeclarationsLtSaftGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtSaftGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/saft/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_ivaz_amend(
        &self,
        request: &PostV1DeclarationsLtIvazAmendRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIvazAmendResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/ivaz/amend",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_ivaz_cancel(
        &self,
        request: &PostV1DeclarationsLtIvazCancelRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIvazCancelResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/ivaz/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_fr0564_compute(
        &self,
        request: &PostV1DeclarationsLtFr0564ComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtFr0564ComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/fr0564/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_gpm312_compute(
        &self,
        request: &PostV1DeclarationsLtGpm312ComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtGpm312ComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/gpm312/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_pln204_compute(
        &self,
        request: &PostV1DeclarationsLtPln204ComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtPln204ComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/pln204/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_oss_compute(
        &self,
        request: &PostV1DeclarationsEuOssComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuOssComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/oss/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_ioss_compute(
        &self,
        request: &PostV1DeclarationsEuIossComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuIossComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/ioss/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_distance_sales_threshold_get(
        &self,
        request: &PostV1DeclarationsEuDistanceSalesThresholdGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuDistanceSalesThresholdGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/distance-sales-threshold/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_union_turnover_get(
        &self,
        request: &PostV1DeclarationsEuUnionTurnoverGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuUnionTurnoverGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/union-turnover/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_sme_cross_border_report_compute(
        &self,
        request: &PostV1DeclarationsEuSmeCrossBorderReportComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuSmeCrossBorderReportComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/sme-cross-border-report/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_sme_thresholds_list(
        &self,
        request: &PostV1DeclarationsEuSmeThresholdsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuSmeThresholdsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/sme-thresholds/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_sme_threshold_get(
        &self,
        request: &PostV1DeclarationsEuSmeThresholdGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuSmeThresholdGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/sme-threshold/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_vat_return_packs_list(
        &self,
        request: &PostV1DeclarationsEuVatReturnPacksListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuVatReturnPacksListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/vat-return/packs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_vat_return_compute(
        &self,
        request: &PostV1DeclarationsEuVatReturnComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuVatReturnComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/vat-return/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate the Polish JPK_V7M(3) file (VAT declaration with evidence) for a month, per the MF schema in force since February 2026. Amounts must already be in PLN; rows are marked BFK until a KSeF integration supplies invoice numbers. Review the warnings before submitting via e-dokumenty.mf.gov.pl.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_jpk_v7_m_generate(
        &self,
        request: &PostV1DeclarationsPlJpkV7MGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlJpkV7MGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/jpk-v7m/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the rows of the Polish recapitulative statement VAT-UE for a month: section C intra-Community supplies of goods, section D intra-Community acquisitions, section E services taxed where the customer is established. Amounts are full złoty per counterparty. The VAT-UE(5) file itself goes out from the EU sales list deadline in the calendar.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_vat_ue_generate(
        &self,
        request: &PostV1DeclarationsPlVatUeGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlVatUeGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/vat-ue/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the rows of the Polish INTRASTAT declaration for a month, arrivals or dispatches, grouped by CN code, partner country, country of origin, partner VAT number, nature of transaction, transport and delivery terms. Values are whole złoty converted at the invoice rate; credit notes with goods lines are returns (code 21). Goods without a CN code are left out and named in the warnings. The IST message itself goes out from the Intrastat deadline in the calendar.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_intrastat_generate(
        &self,
        request: &PostV1DeclarationsPlIntrastatGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlIntrastatGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/intrastat/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the invoices KSeF holds for this company as the buyer, for a window of acquisition timestamps. Each row carries the KSeF number and, when the document number matches a registered purchase invoice, the invoice it belongs to.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_ksef_received_list(
        &self,
        request: &PostV1DeclarationsPlKsefReceivedListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlKsefReceivedListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/ksef/received/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one invoice out of KSeF by its national number. With a purchase invoice given, the KSeF number is written onto that invoice, which is what makes the purchase row of JPK_V7M carry NrKSeF instead of the BFK marker.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_ksef_received_fetch(
        &self,
        request: &PostV1DeclarationsPlKsefReceivedFetchRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlKsefReceivedFetchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/ksef/received/fetch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The UPO for a KSeF session. KSeF issues one receipt per session rather than per invoice, so the session reference number from the send is what identifies it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_ksef_receipt(
        &self,
        request: &PostV1DeclarationsPlKsefReceiptRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlKsefReceiptResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/ksef/receipt",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The differences between the accounting result and the taxable profit: non-deductible expenses, income added to or left out of the tax base, extra deductible expenses, donations, losses carried forward, reliefs and tax credits. The annual corporate income tax return is built from them.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn tax_adjustments_recorded_for_a_tax_year(
        &self,
        request: &PostV1DeclarationsTaxAdjustmentsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsTaxAdjustmentsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/tax-adjustments/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn record_a_tax_adjustment_for_a_tax_year(
        &self,
        request: &PostV1DeclarationsTaxAdjustmentsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsTaxAdjustmentsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/tax-adjustments/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn change_a_recorded_tax_adjustment(
        &self,
        request: &PostV1DeclarationsTaxAdjustmentsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsTaxAdjustmentsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/tax-adjustments/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn remove_a_recorded_tax_adjustment(
        &self,
        request: &PostV1DeclarationsTaxAdjustmentsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsTaxAdjustmentsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/tax-adjustments/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// What the company has paid the administration towards a tax before the return is filed: payments on account, tax withheld at source by others, a final settlement, and a refund received. Returns report these on their own lines, so the amount they ask for is the balance.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn payments_already_made_towards_a_tax_of_a_year(
        &self,
        request: &PostV1DeclarationsTaxPaymentsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsTaxPaymentsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/tax-payments/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn record_a_payment_made_towards_a_tax(
        &self,
        request: &PostV1DeclarationsTaxPaymentsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsTaxPaymentsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/tax-payments/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn change_a_recorded_tax_payment(
        &self,
        request: &PostV1DeclarationsTaxPaymentsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsTaxPaymentsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/tax-payments/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn remove_a_recorded_tax_payment(
        &self,
        request: &PostV1DeclarationsTaxPaymentsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsTaxPaymentsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/tax-payments/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Whether the general meeting adopted the annual accounts and on which date, the date the accounts were prepared, and which directors signed them. The annual accounts filed with the trade register are built from these facts.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn adoption_and_signing_facts_of_the_annual_accounts_of_a_year(
        &self,
        request: &PostV1DeclarationsAnnualAccountsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn record_the_adoption_and_preparation_of_the_annual_accounts_of_a_year(
        &self,
        request: &PostV1DeclarationsAnnualAccountsSetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsSetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn record_whether_a_director_signed_the_annual_accounts_of_a_year(
        &self,
        request: &PostV1DeclarationsAnnualAccountsSignaturesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsSignaturesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/signatures/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn change_a_recorded_director_signature(
        &self,
        request: &PostV1DeclarationsAnnualAccountsSignaturesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsSignaturesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/signatures/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn remove_a_recorded_director_signature(
        &self,
        request: &PostV1DeclarationsAnnualAccountsSignaturesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsSignaturesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/signatures/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn record_a_decision_to_distribute_profit_a_dividend_an_interim_dividend_or_a_payment_treated_as_one(
        &self,
        request: &PostV1DeclarationsAnnualAccountsDistributionsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsDistributionsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/distributions/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn change_a_recorded_profit_distribution(
        &self,
        request: &PostV1DeclarationsAnnualAccountsDistributionsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsDistributionsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/distributions/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn remove_a_recorded_profit_distribution(
        &self,
        request: &PostV1DeclarationsAnnualAccountsDistributionsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsDistributionsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/distributions/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Links a file uploaded through files/upload (its storageKey) to the annual accounts of the year as the notes, the management report, the auditor statement, the profit appropriation resolution, the approval certificate, the general data sheet, the full report as a pdf, or another document. Deposits that must carry these documents take them from here.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn attach_an_uploaded_document_to_the_annual_accounts_of_a_year(
        &self,
        request: &PostV1DeclarationsAnnualAccountsAttachmentsAddRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsAttachmentsAddResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/attachments/add",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn remove_a_document_attached_to_the_annual_accounts_and_delete_its_file(
        &self,
        request: &PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/annual-accounts/attachments/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Compute the company income tax return TD4 of a tax year from the ledger and the recorded tax adjustments: the accounting profit, the add-backs, deductions, capital allowances and losses brought forward, the chargeable income, the corporation tax at the rate of the year and the double tax relief, as the fields the company keys into TAXISnet or Tax For All. The Tax Department publishes no upload layout for the TD4; the XML is a working file.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_cy_td4_generate(
        &self,
        request: &PostV1DeclarationsCyTd4GenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsCyTd4GenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/cy/td4/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the annual return HE32 of a year: the figures the Registrar’s e-filing screens ask for (company number, registered office, made-up-to date, share capital, register of members, directors and secretary, annual general meeting date, the accounts summary), the working file, and the printed form HE32(I) filled in as a PDF for signing and for keying into the Registrar’s system, which takes the return only through its own screens.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_cy_he32_generate(
        &self,
        request: &PostV1DeclarationsCyHe32GenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsCyHe32GenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/cy/he32/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build one of the German returns that ELSTER accepts only through a licensed ERiC transmission (E-Bilanz, Körperschaftsteuer, Gewerbesteuer with its Zerlegungserklärung, annual VAT return, Lohnsteuer-Anmeldung, Lohnsteuerbescheinigung) for the company to send through its own ELSTER-capable program. The period is the year, or YYYY-MM for the monthly Lohnsteuer-Anmeldung.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_de_returns_generate(
        &self,
        request: &PostV1DeclarationsDeReturnsGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsDeReturnsGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/de/returns/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// The facts of one year that the German annual returns (Körperschaftsteuer, Gewerbesteuer, Umsatzsteuererklärung) need and the ledger does not hold: changes of shareholders, contracts with shareholders, the tax contribution account, loss carry-back, the donation carry-forward, the business premises with the municipalities for the apportionment of the trade tax, the land values or property tax and the participations for the trade tax additions and reductions, the foreign income per country for the Anlage AESt, the date of leaving the small-business scheme and the Anlage UN answers of a company seated abroad. A key that is absent has not been answered.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_de_return_facts_get(
        &self,
        request: &PostV1DeclarationsDeReturnFactsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsDeReturnFactsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/de/return-facts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replace the facts of one year for the German annual returns. The returns built afterwards read them; a key left out stays unanswered.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_de_return_facts_set(
        &self,
        request: &PostV1DeclarationsDeReturnFactsSetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/de/return-facts/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the DEÜV notifications of a month (Anmeldung for every start, Abmeldung for every leaving, in December the Jahresmeldung for everyone employed on 31 December) as DSME records with the DBME, DBNA, DBGB and DBAN blocks of Anlage 4 in force from 2026, from the approved payroll runs and the employee record, for the company's own transmission channel.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_de_deuev_generate(
        &self,
        request: &PostV1DeclarationsDeDeuevGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsDeDeuevGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/de/deuev/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the monthly contribution statement to the health insurers (Beitragsnachweis) from the payroll run: one fixed-length record BW02 per insurer, in the record layout in force from 2026, ready for the company's own transmission channel.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_de_beitragsnachweis_generate(
        &self,
        request: &PostV1DeclarationsDeBeitragsnachweisGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsDeBeitragsnachweisGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/de/beitragsnachweis/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Compute the oplysningsskema for selskaber (selskabsselvangivelsen) of an income year from the ledger and the recorded tax adjustments: accounting result before tax, tax adjustments, losses carried forward, taxable income, the 22 % corporation tax, reliefs and the balance, as the rubrikker the company keys into TastSelv Selskabsskat (DIAS). Skatteforvaltningen publishes no file format for the return; the XML is a working file.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_dk_selskabsskat_generate(
        &self,
        request: &PostV1DeclarationsDkSelskabsskatGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsDkSelskabsskatGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/dk/selskabsskat/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send one employment register (töötamise register) entry for an employment contract to e-MTA over X-tee: the start of work, or its end with the reason recorded on the contract.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_ee_employment_register_send(
        &self,
        request: &PostV1DeclarationsEeEmploymentRegisterSendRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEeEmploymentRegisterSendResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/ee/employment-register/send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Nordlet's declaración responsable for its VERI*FACTU invoicing system (Orden HAC/1177/2024, art. 15), as a PDF and as plain text.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_es_verifactu_declaracion_responsable(
        &self,
        request: &PostV1DeclarationsEsVerifactuDeclaracionResponsableRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEsVerifactuDeclaracionResponsableResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/es/verifactu/declaracion-responsable",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the Form CT1 of an accounting year as the ROS version 26 XML and the accompanying financial statements as inline XBRL on the FRS 102 Irish Extension 2026 taxonomy Revenue accepts, both from the ledger, the recorded tax adjustments, the annual accounts record and the officers, for upload through the company’s own ROS account. Says whether the company is above the iXBRL deferral limits (balance sheet total €4.4 million, turnover €8.8 million, 50 employees).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_ie_ct1_generate(
        &self,
        request: &PostV1DeclarationsIeCt1GenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsIeCt1GenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/ie/ct1/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the working paper for the Form B1 annual return of a financial year — company details, registered office, directors and secretary from Settings → Officers, the members from Settings → Shareholders, the issued share capital and the figures of the financial statements — in the order the CORE screens ask for them. The CRO publishes no file format for the B1, so it is keyed into CORE.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_ie_b1_generate(
        &self,
        request: &PostV1DeclarationsIeB1GenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsIeB1GenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/ie/b1/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the TD16-TD19 integration document for a registered purchase invoice and send it to the Sistema di Interscambio. Since July 2022 a purchase from a supplier established abroad is reported this way instead of the esterometro. The Italian VAT rate to self-assess is a judgement about the supply: pass vatRatePercent unless the purchase lines already carry it, otherwise the request is refused rather than guessed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_it_sdi_purchase_send(
        &self,
        request: &PostV1DeclarationsItSdiPurchaseSendRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsItSdiPurchaseSendResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/it/sdi/purchase-send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Render the TD16-TD19 integration document for a registered purchase invoice without sending it, so the rate and the document type can be checked first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_it_sdi_purchase_preview(
        &self,
        request: &PostV1DeclarationsItSdiPurchasePreviewRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsItSdiPurchasePreviewResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/it/sdi/purchase-preview",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Upload the SAF-T file to i.SAF-T over the iSAFTUploaderService web service and start its processing. The submission itself is confirmed separately, because after confirmation the file can no longer be corrected.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_lt_saft_send(
        &self,
        request: &PostV1DeclarationsLtSaftSendRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtSaftSendResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/saft/send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Render the Sodra 1-SD or 2-SD notice for the contracts starting or ending in the range as an .ffdata document for EDAS.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_lt_sd_ffdata(
        &self,
        request: &PostV1DeclarationsLtSdFfdataRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtSdFfdataResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/sd/ffdata",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Render the annual corporate income tax return PLN204 as an .ffdata document, including the PLN204S and PLN204Z annexes, from the ledger and the tax adjustments recorded for that year.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_lt_pln204_ffdata(
        &self,
        request: &PostV1DeclarationsLtPln204FfdataRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtPln204FfdataResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/pln204/ffdata",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Compute the company income tax return and self-assessment of a year of assessment from the ledger and the recorded tax adjustments: the accounting profit before tax, the add-backs and deductions, the approved donations, capital allowances and losses carried forward, the chargeable income, the 35 % charge, the relief against the tax and the allocation of the distributable profit to the five tax accounts. The Malta Tax and Customs Administration issues the return as a personalised spreadsheet to the registered tax practitioner and publishes no layout, so the XML is a working file and the figures are keyed into that spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_mt_company_tax_generate(
        &self,
        request: &PostV1DeclarationsMtCompanyTaxGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsMtCompanyTaxGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/mt/company-tax/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the annual return of a year: the company number, registered office and made-up-to date, the share capital, the register of members, the directors and the company secretary and the accounts summary, as the figures the Malta Business Registry asks for on its own screens, plus the printed Annual Return Form of the Seventh Schedule filled in as a PDF for signing.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_mt_annual_return_generate(
        &self,
        request: &PostV1DeclarationsMtAnnualReturnGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsMtAnnualReturnGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/mt/annual-return/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate JPK_FA(4), the on-demand structure with every sales invoice issued in a period, its VAT bases per rate and one row per invoice line. Filed only when the tax office asks for it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_jpk_fa_generate(
        &self,
        request: &PostV1DeclarationsPlJpkFaGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlJpkFaGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/jpk-fa/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate JPK_KR(1), the on-demand structure with the chart of accounts and its opening balances and turnover, the journal and the double entries behind it. Filed only when the tax office asks for it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_jpk_kr_generate(
        &self,
        request: &PostV1DeclarationsPlJpkKrGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlJpkKrGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/jpk-kr/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate JPK_MAG(2), the on-demand structure with the warehouse documents of one warehouse: goods received from outside (PZ) or internally (PW) and issued to a customer (WZ) or internally (RW). Filed only when the tax office asks for it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_jpk_mag_generate(
        &self,
        request: &PostV1DeclarationsPlJpkMagGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlJpkMagGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/jpk-mag/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate PIT-11(29) for every person on the payroll of one year: the pay, the deductible costs, the advance withheld and the social and health contributions taken off it. One document per person, because that is how the form is filed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_pit11_generate(
        &self,
        request: &PostV1DeclarationsPlPit11GenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlPit11GenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/pit-11/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generate CIT-8(34), the annual corporate income tax return, from the ledger of the year and the recorded tax adjustments. The tax office code and the small-taxpayer setting come from the e-Deklaracje compliance settings, the seat address from the JPK gateway settings. Names the annexes the figures would need, which are not produced.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_cit8_generate(
        &self,
        request: &PostV1DeclarationsPlCit8GenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlCit8GenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/cit-8/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Compute the monthly ZUS DRA settlement from the payroll run of one month: the pension, disability, sickness, accident and health insurance contributions and the Labour Fund, Solidarity Fund and guaranteed benefits fund charges, each split between the insured person and the payer. The amounts are carried into Płatnik or ePłatnik by hand.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_zus_dra_compute(
        &self,
        request: &PostV1DeclarationsPlZusDraComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlZusDraComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/zus-dra/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the KEDU file for one month: the ZUS DRA settlement and one ZUS RCA report per person on the payroll, in the schema kedu_5_4 that Płatnik and ePłatnik import. The payer REGON, short name and declaration deadline code come from the ZUS compliance settings; the insurance title code and working time of each person from the employee record.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_zus_dra_kedu(
        &self,
        request: &PostV1DeclarationsPlZusDraKeduRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlZusDraKeduResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/zus-dra/kedu",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fill the published ZUS DRA form for one month and return it as a PDF. The amounts, the payer identity and the deadline code are the same ones the KEDU file carries; blocks the payroll does not hold (paid benefits, bridging pensions, income declaration of a self-paying person) stay empty.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_pl_zus_dra_pdf(
        &self,
        request: &PostV1DeclarationsPlZusDraPdfRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsPlZusDraPdfResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/pl/zus-dra/pdf",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the RO e-Transport declaration for an issued waybill: goods with their tariff codes and masses, the commercial partner, the route and the vehicle. The XML follows the ANAF eTransport v2 schema and is kept as a file on the waybill. Anything listed in blockers has to be filled in before /etransport/send will accept it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_ro_etransport_build(
        &self,
        request: &PostV1DeclarationsRoEtransportBuildRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsRoEtransportBuildResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/ro/etransport/build",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Hand the RO e-Transport declaration for an issued waybill to ANAF under the SPV OAuth token in compliance settings, and return the upload index the UIT is read back with. Answers 422 while any field the ANAF validator requires is still missing.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_ro_etransport_submit(
        &self,
        request: &PostV1DeclarationsRoEtransportSubmitRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsRoEtransportSubmitResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/ro/etransport/submit",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read the outcome of an e-Transport declaration from ANAF by its upload index, under the SPV OAuth token in compliance settings. Returns the UIT code once the declaration validates.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_ro_etransport_status(
        &self,
        request: &PostV1DeclarationsRoEtransportStatusRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsRoEtransportStatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/ro/etransport/status",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the annual wage declaration (Lohndeklaration) to the AHV-IV-FAK from the approved payroll runs of the year as the CSV that AHVeasy imports under Lohndeklaration → CSV-Import der Lohndaten: one row per employee with the 18 columns of the AHVeasy template, the AHV-liable wage and the ALV wage.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_li_lohndeklaration_generate(
        &self,
        request: &PostV1DeclarationsLiLohndeklarationGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLiLohndeklarationGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/li/lohndeklaration/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the annual wage list (Lohnliste) of a Liechtenstein employer from the approved payroll runs of the year as the XLSX file the tax administration's eLohnausweis / eLohnlisten application imports: one row per employee with PEID, name, birth date, address, gross wage, wage tax withheld and the settlement period.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_declarations_li_lohnlisten_generate(
        &self,
        request: &PostV1DeclarationsLiLohnlistenGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLiLohnlistenGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/li/lohnlisten/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_configs_list(
        &self,
        request: &PostV1DeclarationsConfigsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsConfigsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/configs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_configs_update(
        &self,
        request: &PostV1DeclarationsConfigsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsConfigsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/configs/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn store_the_certificate_or_private_key_a_filing_system_authenticates_with(
        &self,
        request: &PostV1DeclarationsCertificatesUploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsCertificatesUploadResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/certificates/upload",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_certificates_list(
        &self,
        request: &PostV1DeclarationsCertificatesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsCertificatesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/certificates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_certificates_delete(
        &self,
        request: &PostV1DeclarationsCertificatesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsCertificatesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/certificates/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn which_deadlines_nordlet_can_file_by_itself_for_this_company_and_which_are_switched_on(
        &self,
        request: &PostV1DeclarationsAutomationListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAutomationListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/automation/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_automation_update(
        &self,
        request: &PostV1DeclarationsAutomationUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsAutomationUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/automation/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn send_a_filing_whose_delivery_failed_once_more_with_the_bytes_that_were_generated(
        &self,
        request: &PostV1DeclarationsSubmissionsRetryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsSubmissionsRetryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/submissions/retry",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_submissions_create(
        &self,
        request: &PostV1DeclarationsSubmissionsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsSubmissionsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/submissions/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_submissions_mark(
        &self,
        request: &PostV1DeclarationsSubmissionsMarkRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsSubmissionsMarkResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/submissions/mark",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_submissions_list(
        &self,
        request: &PostV1DeclarationsSubmissionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsSubmissionsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/submissions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
