pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvoicesCreateSalesRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<InvoicesCreateSalesRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<NaiveDate>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(rename = "creditedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_id: Option<String>,
    /// Number of an original invoice issued outside Nordlet; give it with creditedInvoiceDate
    #[serde(rename = "creditedInvoiceReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_reference: Option<String>,
    /// Issue date of the original invoice issued outside Nordlet
    #[serde(rename = "creditedInvoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_date: Option<NaiveDate>,
    #[serde(rename = "agreementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(rename = "vatScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_scheme: Option<InvoicesCreateSalesRequestVatScheme>,
    #[serde(rename = "intrastatTransportMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrastat_transport_mode: Option<String>,
    #[serde(rename = "intrastatDeliveryTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrastat_delivery_terms: Option<String>,
    #[serde(rename = "intrastatRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrastat_region: Option<String>,
    #[serde(rename = "intrastatNatureOfTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrastat_nature_of_transaction: Option<String>,
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "deemedSupplier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deemed_supplier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
    #[serde(rename = "operationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type_id: Option<String>,
    #[serde(rename = "documentSeriesId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_series_id: Option<String>,
    #[serde(rename = "seriesLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_label: Option<String>,
    #[serde(rename = "orderNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(rename = "issuedByName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_by_name: Option<String>,
    #[serde(rename = "issuedByTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_by_title: Option<String>,
    #[serde(rename = "receivedByName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_by_name: Option<String>,
    #[serde(rename = "receivedByTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_by_title: Option<String>,
    #[serde(rename = "discountPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<String>,
    #[serde(default)]
    pub lines: Vec<InvoicesCreateSalesRequestLinesItem>,
}

impl InvoicesCreateSalesRequest {
    pub fn builder() -> InvoicesCreateSalesRequestBuilder {
        <InvoicesCreateSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCreateSalesRequestBuilder {
    partner_id: Option<String>,
    r#type: Option<InvoicesCreateSalesRequestType>,
    currency: Option<String>,
    issue_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    credited_invoice_id: Option<String>,
    credited_invoice_reference: Option<String>,
    credited_invoice_date: Option<NaiveDate>,
    agreement_id: Option<String>,
    vat_scheme: Option<InvoicesCreateSalesRequestVatScheme>,
    intrastat_transport_mode: Option<String>,
    intrastat_delivery_terms: Option<String>,
    intrastat_region: Option<String>,
    intrastat_nature_of_transaction: Option<String>,
    vat_country_code: Option<String>,
    deemed_supplier: Option<bool>,
    notes: Option<String>,
    document_ref: Option<String>,
    operation_type_id: Option<String>,
    document_series_id: Option<String>,
    series_label: Option<String>,
    order_number: Option<String>,
    issued_by_name: Option<String>,
    issued_by_title: Option<String>,
    received_by_name: Option<String>,
    received_by_title: Option<String>,
    discount_percent: Option<String>,
    lines: Option<Vec<InvoicesCreateSalesRequestLinesItem>>,
}

impl InvoicesCreateSalesRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: InvoicesCreateSalesRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn credited_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.credited_invoice_id = Some(value.into());
        self
    }

    pub fn credited_invoice_reference(mut self, value: impl Into<String>) -> Self {
        self.credited_invoice_reference = Some(value.into());
        self
    }

    pub fn credited_invoice_date(mut self, value: NaiveDate) -> Self {
        self.credited_invoice_date = Some(value);
        self
    }

    pub fn agreement_id(mut self, value: impl Into<String>) -> Self {
        self.agreement_id = Some(value.into());
        self
    }

    pub fn vat_scheme(mut self, value: InvoicesCreateSalesRequestVatScheme) -> Self {
        self.vat_scheme = Some(value);
        self
    }

    pub fn intrastat_transport_mode(mut self, value: impl Into<String>) -> Self {
        self.intrastat_transport_mode = Some(value.into());
        self
    }

    pub fn intrastat_delivery_terms(mut self, value: impl Into<String>) -> Self {
        self.intrastat_delivery_terms = Some(value.into());
        self
    }

    pub fn intrastat_region(mut self, value: impl Into<String>) -> Self {
        self.intrastat_region = Some(value.into());
        self
    }

    pub fn intrastat_nature_of_transaction(mut self, value: impl Into<String>) -> Self {
        self.intrastat_nature_of_transaction = Some(value.into());
        self
    }

    pub fn vat_country_code(mut self, value: impl Into<String>) -> Self {
        self.vat_country_code = Some(value.into());
        self
    }

    pub fn deemed_supplier(mut self, value: bool) -> Self {
        self.deemed_supplier = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
        self
    }

    pub fn operation_type_id(mut self, value: impl Into<String>) -> Self {
        self.operation_type_id = Some(value.into());
        self
    }

    pub fn document_series_id(mut self, value: impl Into<String>) -> Self {
        self.document_series_id = Some(value.into());
        self
    }

    pub fn series_label(mut self, value: impl Into<String>) -> Self {
        self.series_label = Some(value.into());
        self
    }

    pub fn order_number(mut self, value: impl Into<String>) -> Self {
        self.order_number = Some(value.into());
        self
    }

    pub fn issued_by_name(mut self, value: impl Into<String>) -> Self {
        self.issued_by_name = Some(value.into());
        self
    }

    pub fn issued_by_title(mut self, value: impl Into<String>) -> Self {
        self.issued_by_title = Some(value.into());
        self
    }

    pub fn received_by_name(mut self, value: impl Into<String>) -> Self {
        self.received_by_name = Some(value.into());
        self
    }

    pub fn received_by_title(mut self, value: impl Into<String>) -> Self {
        self.received_by_title = Some(value.into());
        self
    }

    pub fn discount_percent(mut self, value: impl Into<String>) -> Self {
        self.discount_percent = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<InvoicesCreateSalesRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCreateSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](InvoicesCreateSalesRequestBuilder::partner_id)
    /// - [`lines`](InvoicesCreateSalesRequestBuilder::lines)
    pub fn build(self) -> Result<InvoicesCreateSalesRequest, BuildError> {
        Ok(InvoicesCreateSalesRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            r#type: self.r#type,
            currency: self.currency,
            issue_date: self.issue_date,
            due_date: self.due_date,
            credited_invoice_id: self.credited_invoice_id,
            credited_invoice_reference: self.credited_invoice_reference,
            credited_invoice_date: self.credited_invoice_date,
            agreement_id: self.agreement_id,
            vat_scheme: self.vat_scheme,
            intrastat_transport_mode: self.intrastat_transport_mode,
            intrastat_delivery_terms: self.intrastat_delivery_terms,
            intrastat_region: self.intrastat_region,
            intrastat_nature_of_transaction: self.intrastat_nature_of_transaction,
            vat_country_code: self.vat_country_code,
            deemed_supplier: self.deemed_supplier,
            notes: self.notes,
            document_ref: self.document_ref,
            operation_type_id: self.operation_type_id,
            document_series_id: self.document_series_id,
            series_label: self.series_label,
            order_number: self.order_number,
            issued_by_name: self.issued_by_name,
            issued_by_title: self.issued_by_title,
            received_by_name: self.received_by_name,
            received_by_title: self.received_by_title,
            discount_percent: self.discount_percent,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
