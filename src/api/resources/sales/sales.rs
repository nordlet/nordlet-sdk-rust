use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SalesClient {
    pub http_client: HttpClient,
}

impl SalesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn invoices_create(
        &self,
        request: &InvoicesCreateSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesCreateSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_get(
        &self,
        request: &InvoicesGetSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesGetSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_pdf(
        &self,
        request: &InvoicesPdfSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesPdfSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/pdf",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_send(
        &self,
        request: &InvoicesSendSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesSendSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_peppol_xml(
        &self,
        request: &InvoicesPeppolXmlSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesPeppolXmlSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/peppol-xml",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send an issued invoice or credit note to the customer over Peppol through the company's own access point (Settings → Compliance → EU; Nordlet supports Recommand, Storecove and e-invoice.be). Without one the call is refused with 422 and the document can only be downloaded with `sales/invoices/peppol-xml`. `status` is `pending` until the receiving access point confirms, then `delivered`; `failed` and `rejected` come with `detail`, and the invoice can then be sent again. Later changes arrive through the access point's webhook and are announced as `sale_invoice.peppol_delivered`, `sale_invoice.peppol_rejected` and `sale_invoice.peppol_failed`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_peppol_send(
        &self,
        request: &InvoicesPeppolSendSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesPeppolSendSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/peppol-send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Ask the company's Peppol access point what happened to an invoice sent with `sales/invoices/peppol-send`, and store the answer: `pending`, `delivered` (the receiving access point confirmed it), `rejected` (the receiver refused it, see `detail`) or `failed` (it could not be delivered, see `detail`). The access point's webhook updates the same fields without this call. Storecove has no call for the status of a sent document, so for a Storecove access point this answers 422 and the status comes only from its webhook.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_peppol_status(
        &self,
        request: &InvoicesPeppolStatusSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesPeppolStatusSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/peppol-status",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Render an issued invoice as the national e-invoicing payload for the company country: FatturaPA (IT), KSeF FA(3) (PL) or UBL CIUS-RO (RO). Review the warnings - data the invoice does not carry is flagged, never invented.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_einvoice_xml(
        &self,
        request: &InvoicesEinvoiceXmlSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesEinvoiceXmlSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/einvoice-xml",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Build the national e-invoicing payload and deliver it over the transport configured for the country gateway in compliance settings. With transport=direct the request talks to the tax authority itself - SdICoop over 2-way TLS for Italy, a KSeF session for Poland, ANAF SPV OAuth for Romania - and returns the national number as soon as the channel assigns one. With transport=bridge the payload goes to the configured bridge endpoint (an accredited intermediary or connector) instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_einvoice_send(
        &self,
        request: &InvoicesEinvoiceSendSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesEinvoiceSendSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/einvoice-send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Ask the national e-invoicing channel what happened to an invoice that was already sent, and store the answer. Italy, Poland and Romania return the outcome only on request - none of them calls back - so this is the way the national number and any rejection reason reach the invoice.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_einvoice_status(
        &self,
        request: &InvoicesEinvoiceStatusSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesEinvoiceStatusSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/einvoice-status",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_update(
        &self,
        request: &InvoicesUpdateSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesUpdateSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_delete(
        &self,
        request: &InvoicesDeleteSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesDeleteSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_issue(
        &self,
        request: &InvoicesIssueSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesIssueSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/issue",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_lock(
        &self,
        request: &InvoicesLockSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesLockSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/lock",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_unlock(
        &self,
        request: &InvoicesUnlockSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesUnlockSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/unlock",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_payment_link(
        &self,
        request: &InvoicesPaymentLinkSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesPaymentLinkSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/payment-link",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_payment_settings_get(
        &self,
        request: &InvoicesPaymentSettingsGetSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesPaymentSettingsGetSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/payment-settings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_payment_settings_update(
        &self,
        request: &InvoicesPaymentSettingsUpdateSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesPaymentSettingsUpdateSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/payment-settings/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn recognition_schedules_list(
        &self,
        request: &RecognitionSchedulesListSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecognitionSchedulesListSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition-schedules/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_apply_advance(
        &self,
        request: &InvoicesApplyAdvanceSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesApplyAdvanceSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/apply-advance",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn invoices_list(
        &self,
        request: &InvoicesListSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesListSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/invoices/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn acts_create(
        &self,
        request: &ActsCreateSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ActsCreateSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn acts_update(
        &self,
        request: &ActsUpdateSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ActsUpdateSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn acts_issue(
        &self,
        request: &ActsIssueSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ActsIssueSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/issue",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn acts_cancel(
        &self,
        request: &ActsCancelSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ActsCancelSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn acts_get(
        &self,
        request: &ActsGetSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ActsGetSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn acts_list(
        &self,
        request: &ActsListSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ActsListSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn acts_pdf(
        &self,
        request: &ActsPdfSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ActsPdfSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/acts/pdf",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn recognition_compute(
        &self,
        request: &RecognitionComputeSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecognitionComputeSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn recognition_run(
        &self,
        request: &RecognitionRunSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecognitionRunSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/run",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn recognition_progress(
        &self,
        request: &RecognitionProgressSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecognitionProgressSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/progress",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Apply an IFRS 15 contract modification to a deferred invoice line. Prospective: cancel the pending schedule and respread the unrecognized remainder over the new terms. Cumulative catch-up (ratable only): recompute revenue as if the new terms applied from the start and post the difference immediately.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn recognition_modify(
        &self,
        request: &RecognitionModifySalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecognitionModifySalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/modify",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn recognition_runs_list(
        &self,
        request: &RecognitionRunsListSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecognitionRunsListSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/runs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn recognition_summary(
        &self,
        request: &RecognitionSummarySalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecognitionSummarySalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/recognition/summary",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn refund_liability_list(
        &self,
        request: &RefundLiabilityListSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RefundLiabilityListSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/refund-liability/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn refund_liability_true_up(
        &self,
        request: &RefundLiabilityTrueUpSalesRequest,
        options: Option<RequestOptions>,
    ) -> Result<RefundLiabilityTrueUpSalesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/sales/refund-liability/true-up",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
