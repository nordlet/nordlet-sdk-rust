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

    pub async fn lt_intrastat_compute(
        &self,
        request: &LtIntrastatComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtIntrastatComputeDeclarationsResponse, ApiError> {
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

    pub async fn lt_ivaz_generate(
        &self,
        request: &LtIvazGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtIvazGenerateDeclarationsResponse, ApiError> {
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

    pub async fn lt_intrastat_obligation(
        &self,
        request: &LtIntrastatObligationDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtIntrastatObligationDeclarationsResponse, ApiError> {
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

    pub async fn lt_isaf_generate(
        &self,
        request: &LtIsafGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtIsafGenerateDeclarationsResponse, ApiError> {
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

    pub async fn lt_fr0600_compute(
        &self,
        request: &LtFr0600ComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtFr0600ComputeDeclarationsResponse, ApiError> {
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

    pub async fn lt_gpm313_compute(
        &self,
        request: &LtGpm313ComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtGpm313ComputeDeclarationsResponse, ApiError> {
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

    pub async fn lt_sam_compute(
        &self,
        request: &LtSamComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtSamComputeDeclarationsResponse, ApiError> {
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

    pub async fn lt_sd_generate(
        &self,
        request: &LtSdGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtSdGenerateDeclarationsResponse, ApiError> {
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

    pub async fn lt_saft_generate(
        &self,
        request: &LtSaftGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtSaftGenerateDeclarationsResponse, ApiError> {
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

    pub async fn lt_ivaz_amend(
        &self,
        request: &LtIvazAmendDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtIvazAmendDeclarationsResponse, ApiError> {
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

    pub async fn lt_ivaz_cancel(
        &self,
        request: &LtIvazCancelDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtIvazCancelDeclarationsResponse, ApiError> {
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

    pub async fn lt_fr0564_compute(
        &self,
        request: &LtFr0564ComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtFr0564ComputeDeclarationsResponse, ApiError> {
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

    pub async fn lt_gpm312_compute(
        &self,
        request: &LtGpm312ComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtGpm312ComputeDeclarationsResponse, ApiError> {
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

    pub async fn lt_pln204_compute(
        &self,
        request: &LtPln204ComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtPln204ComputeDeclarationsResponse, ApiError> {
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

    pub async fn eu_oss_compute(
        &self,
        request: &EuOssComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuOssComputeDeclarationsResponse, ApiError> {
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

    pub async fn eu_ioss_compute(
        &self,
        request: &EuIossComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuIossComputeDeclarationsResponse, ApiError> {
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

    pub async fn eu_own_goods_transfers_compute(
        &self,
        request: &EuOwnGoodsTransfersComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuOwnGoodsTransfersComputeDeclarationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/own-goods-transfers/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn eu_digital_reporting_list(
        &self,
        request: &EuDigitalReportingListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuDigitalReportingListDeclarationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/digital-reporting/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Which platform sellers are reportable for the year (Council Directive (EU) 2021/514, Annex V) and why the others are excluded, the data still missing, and how the company files the report in its Member State.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn eu_dac7_preview(
        &self,
        request: &EuDac7PreviewDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuDac7PreviewDeclarationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/dac7/preview",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn eu_dac7_xml(
        &self,
        request: &EuDac7XmlDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuDac7XmlDeclarationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/dac7/xml",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn eu_distance_sales_threshold_get(
        &self,
        request: &EuDistanceSalesThresholdGetDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuDistanceSalesThresholdGetDeclarationsResponse, ApiError> {
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

    pub async fn eu_union_turnover_get(
        &self,
        request: &EuUnionTurnoverGetDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuUnionTurnoverGetDeclarationsResponse, ApiError> {
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

    pub async fn eu_sme_cross_border_report_compute(
        &self,
        request: &EuSmeCrossBorderReportComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuSmeCrossBorderReportComputeDeclarationsResponse, ApiError> {
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

    pub async fn eu_sme_thresholds_list(
        &self,
        request: &EuSmeThresholdsListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuSmeThresholdsListDeclarationsResponse, ApiError> {
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

    pub async fn eu_sme_threshold_get(
        &self,
        request: &EuSmeThresholdGetDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuSmeThresholdGetDeclarationsResponse, ApiError> {
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

    pub async fn eu_vat_return_packs_list(
        &self,
        request: &EuVatReturnPacksListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuVatReturnPacksListDeclarationsResponse, ApiError> {
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

    pub async fn eu_vat_return_compute(
        &self,
        request: &EuVatReturnComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuVatReturnComputeDeclarationsResponse, ApiError> {
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
    pub async fn pl_jpk_v7m_generate(
        &self,
        request: &PlJpkV7MGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlJpkV7MGenerateDeclarationsResponse, ApiError> {
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
    pub async fn pl_vat_ue_generate(
        &self,
        request: &PlVatUeGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlVatUeGenerateDeclarationsResponse, ApiError> {
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
    pub async fn pl_intrastat_generate(
        &self,
        request: &PlIntrastatGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlIntrastatGenerateDeclarationsResponse, ApiError> {
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
    pub async fn pl_ksef_received_list(
        &self,
        request: &PlKsefReceivedListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlKsefReceivedListDeclarationsResponse, ApiError> {
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
    pub async fn pl_ksef_received_fetch(
        &self,
        request: &PlKsefReceivedFetchDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlKsefReceivedFetchDeclarationsResponse, ApiError> {
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
    pub async fn pl_ksef_receipt(
        &self,
        request: &PlKsefReceiptDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlKsefReceiptDeclarationsResponse, ApiError> {
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
    pub async fn tax_adjustments_list(
        &self,
        request: &TaxAdjustmentsListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxAdjustmentsListDeclarationsResponse, ApiError> {
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

    pub async fn tax_adjustments_create(
        &self,
        request: &TaxAdjustmentsCreateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxAdjustmentsCreateDeclarationsResponse, ApiError> {
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

    pub async fn tax_adjustments_update(
        &self,
        request: &TaxAdjustmentsUpdateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxAdjustmentsUpdateDeclarationsResponse, ApiError> {
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

    pub async fn tax_adjustments_delete(
        &self,
        request: &TaxAdjustmentsDeleteDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxAdjustmentsDeleteDeclarationsResponse, ApiError> {
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
    pub async fn tax_payments_list(
        &self,
        request: &TaxPaymentsListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxPaymentsListDeclarationsResponse, ApiError> {
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

    pub async fn tax_payments_create(
        &self,
        request: &TaxPaymentsCreateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxPaymentsCreateDeclarationsResponse, ApiError> {
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

    pub async fn tax_payments_update(
        &self,
        request: &TaxPaymentsUpdateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxPaymentsUpdateDeclarationsResponse, ApiError> {
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

    pub async fn tax_payments_delete(
        &self,
        request: &TaxPaymentsDeleteDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxPaymentsDeleteDeclarationsResponse, ApiError> {
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
    pub async fn annual_accounts_get(
        &self,
        request: &AnnualAccountsGetDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsGetDeclarationsResponse, ApiError> {
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

    pub async fn annual_accounts_set(
        &self,
        request: &AnnualAccountsSetDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsSetDeclarationsResponse, ApiError> {
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

    pub async fn annual_accounts_signatures_create(
        &self,
        request: &AnnualAccountsSignaturesCreateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsSignaturesCreateDeclarationsResponse, ApiError> {
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

    pub async fn annual_accounts_signatures_update(
        &self,
        request: &AnnualAccountsSignaturesUpdateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsSignaturesUpdateDeclarationsResponse, ApiError> {
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

    pub async fn annual_accounts_signatures_delete(
        &self,
        request: &AnnualAccountsSignaturesDeleteDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsSignaturesDeleteDeclarationsResponse, ApiError> {
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

    pub async fn annual_accounts_distributions_create(
        &self,
        request: &AnnualAccountsDistributionsCreateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsDistributionsCreateDeclarationsResponse, ApiError> {
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

    pub async fn annual_accounts_distributions_update(
        &self,
        request: &AnnualAccountsDistributionsUpdateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsDistributionsUpdateDeclarationsResponse, ApiError> {
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

    pub async fn annual_accounts_distributions_delete(
        &self,
        request: &AnnualAccountsDistributionsDeleteDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsDistributionsDeleteDeclarationsResponse, ApiError> {
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
    pub async fn annual_accounts_attachments_add(
        &self,
        request: &AnnualAccountsAttachmentsAddDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsAttachmentsAddDeclarationsResponse, ApiError> {
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

    pub async fn annual_accounts_attachments_delete(
        &self,
        request: &AnnualAccountsAttachmentsDeleteDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnnualAccountsAttachmentsDeleteDeclarationsResponse, ApiError> {
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
    pub async fn cy_td4_generate(
        &self,
        request: &CyTd4GenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CyTd4GenerateDeclarationsResponse, ApiError> {
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
    pub async fn cy_he32_generate(
        &self,
        request: &CyHe32GenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CyHe32GenerateDeclarationsResponse, ApiError> {
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
    pub async fn de_returns_generate(
        &self,
        request: &DeReturnsGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeReturnsGenerateDeclarationsResponse, ApiError> {
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
    pub async fn de_return_facts_get(
        &self,
        request: &DeReturnFactsGetDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeReturnFactsGetDeclarationsResponse, ApiError> {
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
    pub async fn de_return_facts_set(
        &self,
        request: &DeReturnFactsSetDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeReturnFactsSetDeclarationsResponse, ApiError> {
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
    pub async fn de_deuev_generate(
        &self,
        request: &DeDeuevGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeDeuevGenerateDeclarationsResponse, ApiError> {
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
    pub async fn de_beitragsnachweis_generate(
        &self,
        request: &DeBeitragsnachweisGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeBeitragsnachweisGenerateDeclarationsResponse, ApiError> {
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
    pub async fn dk_selskabsskat_generate(
        &self,
        request: &DkSelskabsskatGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DkSelskabsskatGenerateDeclarationsResponse, ApiError> {
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
    pub async fn ee_employment_register_send(
        &self,
        request: &EeEmploymentRegisterSendDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EeEmploymentRegisterSendDeclarationsResponse, ApiError> {
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
    pub async fn es_verifactu_declaracion_responsable(
        &self,
        request: &EsVerifactuDeclaracionResponsableDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EsVerifactuDeclaracionResponsableDeclarationsResponse, ApiError> {
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
    pub async fn ie_ct1_generate(
        &self,
        request: &IeCt1GenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<IeCt1GenerateDeclarationsResponse, ApiError> {
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

    /// Build the working paper for the Form B1 annual return of a financial year - company details, registered office, directors and secretary from Settings → Officers, the members from Settings → Shareholders, the issued share capital and the figures of the financial statements - in the order the CORE screens ask for them. The CRO publishes no file format for the B1, so it is keyed into CORE.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn ie_b1_generate(
        &self,
        request: &IeB1GenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<IeB1GenerateDeclarationsResponse, ApiError> {
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
    pub async fn it_sdi_purchase_send(
        &self,
        request: &ItSdiPurchaseSendDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItSdiPurchaseSendDeclarationsResponse, ApiError> {
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
    pub async fn it_sdi_purchase_preview(
        &self,
        request: &ItSdiPurchasePreviewDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItSdiPurchasePreviewDeclarationsResponse, ApiError> {
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

    /// Upload the SAF-T file to i.SAF-T over the iSAFTUploaderService web service and start its processing. The file, the case reference and the status are kept as a declaration submission (submissionId), whose outcome Nordlet then checks with i.SAF-T. The submission itself is confirmed separately, because after confirmation the file can no longer be corrected. A range and data type already sent is sent again only with amend: true.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn lt_saft_send(
        &self,
        request: &LtSaftSendDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtSaftSendDeclarationsResponse, ApiError> {
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
    pub async fn lt_sd_ffdata(
        &self,
        request: &LtSdFfdataDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtSdFfdataDeclarationsResponse, ApiError> {
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
    pub async fn lt_pln204_ffdata(
        &self,
        request: &LtPln204FfdataDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtPln204FfdataDeclarationsResponse, ApiError> {
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
    pub async fn mt_company_tax_generate(
        &self,
        request: &MtCompanyTaxGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MtCompanyTaxGenerateDeclarationsResponse, ApiError> {
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
    pub async fn mt_annual_return_generate(
        &self,
        request: &MtAnnualReturnGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MtAnnualReturnGenerateDeclarationsResponse, ApiError> {
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
    pub async fn pl_jpk_fa_generate(
        &self,
        request: &PlJpkFaGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlJpkFaGenerateDeclarationsResponse, ApiError> {
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
    pub async fn pl_jpk_kr_generate(
        &self,
        request: &PlJpkKrGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlJpkKrGenerateDeclarationsResponse, ApiError> {
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
    pub async fn pl_jpk_mag_generate(
        &self,
        request: &PlJpkMagGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlJpkMagGenerateDeclarationsResponse, ApiError> {
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

    /// Generate PIT-11(29) for every person on the payroll of one year: the pay, the deductible costs, the advance withheld and the social and health contributions taken off it. One document per person, because that is how the form is filed, addressed to the tax office of the place of residence of that person (employee field plKodUrzedu); a person without that code is refused with 422.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn pl_pit11_generate(
        &self,
        request: &PlPit11GenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlPit11GenerateDeclarationsResponse, ApiError> {
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
    pub async fn pl_cit8_generate(
        &self,
        request: &PlCit8GenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlCit8GenerateDeclarationsResponse, ApiError> {
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
    pub async fn pl_zus_dra_compute(
        &self,
        request: &PlZusDraComputeDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlZusDraComputeDeclarationsResponse, ApiError> {
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
    pub async fn pl_zus_dra_kedu(
        &self,
        request: &PlZusDraKeduDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlZusDraKeduDeclarationsResponse, ApiError> {
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
    pub async fn pl_zus_dra_pdf(
        &self,
        request: &PlZusDraPdfDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlZusDraPdfDeclarationsResponse, ApiError> {
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
    pub async fn ro_etransport_build(
        &self,
        request: &RoEtransportBuildDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoEtransportBuildDeclarationsResponse, ApiError> {
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
    pub async fn ro_etransport_submit(
        &self,
        request: &RoEtransportSubmitDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoEtransportSubmitDeclarationsResponse, ApiError> {
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
    pub async fn ro_etransport_status(
        &self,
        request: &RoEtransportStatusDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoEtransportStatusDeclarationsResponse, ApiError> {
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
    pub async fn li_lohndeklaration_generate(
        &self,
        request: &LiLohndeklarationGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LiLohndeklarationGenerateDeclarationsResponse, ApiError> {
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
    pub async fn li_lohnlisten_generate(
        &self,
        request: &LiLohnlistenGenerateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LiLohnlistenGenerateDeclarationsResponse, ApiError> {
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

    pub async fn configs_list(
        &self,
        request: &ConfigsListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConfigsListDeclarationsResponse, ApiError> {
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

    pub async fn configs_update(
        &self,
        request: &ConfigsUpdateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConfigsUpdateDeclarationsResponse, ApiError> {
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

    pub async fn certificates_upload(
        &self,
        request: &CertificatesUploadDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CertificatesUploadDeclarationsResponse, ApiError> {
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

    pub async fn certificates_list(
        &self,
        request: &CertificatesListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CertificatesListDeclarationsResponse, ApiError> {
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

    pub async fn certificates_delete(
        &self,
        request: &CertificatesDeleteDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CertificatesDeleteDeclarationsResponse, ApiError> {
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

    pub async fn automation_list(
        &self,
        request: &AutomationListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AutomationListDeclarationsResponse, ApiError> {
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

    pub async fn automation_update(
        &self,
        request: &AutomationUpdateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AutomationUpdateDeclarationsResponse, ApiError> {
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

    pub async fn submissions_retry(
        &self,
        request: &SubmissionsRetryDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmissionsRetryDeclarationsResponse, ApiError> {
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

    pub async fn submissions_create(
        &self,
        request: &SubmissionsCreateDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmissionsCreateDeclarationsResponse, ApiError> {
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

    pub async fn submissions_mark(
        &self,
        request: &SubmissionsMarkDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmissionsMarkDeclarationsResponse, ApiError> {
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

    pub async fn submissions_list(
        &self,
        request: &SubmissionsListDeclarationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmissionsListDeclarationsResponse, ApiError> {
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
