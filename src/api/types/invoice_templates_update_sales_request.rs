pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceTemplatesUpdateSalesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "documentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<InvoiceTemplatesUpdateSalesRequestDocumentType>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "paymentTermDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term_days: Option<i64>,
    #[serde(rename = "vatScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_scheme: Option<InvoiceTemplatesUpdateSalesRequestVatScheme>,
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "agreementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(rename = "operationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type_id: Option<String>,
    #[serde(rename = "documentSeriesId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_series_id: Option<String>,
    #[serde(rename = "discountPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percent: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "supplyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_type: Option<InvoiceTemplatesUpdateSalesRequestSupplyType>,
    #[serde(rename = "seriesLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_label: Option<String>,
    #[serde(rename = "orderNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<InvoiceTemplatesUpdateSalesRequestLinesItem>>,
}

impl InvoiceTemplatesUpdateSalesRequest {
    pub fn builder() -> InvoiceTemplatesUpdateSalesRequestBuilder {
        <InvoiceTemplatesUpdateSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceTemplatesUpdateSalesRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    document_type: Option<InvoiceTemplatesUpdateSalesRequestDocumentType>,
    partner_id: Option<String>,
    currency: Option<String>,
    payment_term_days: Option<i64>,
    vat_scheme: Option<InvoiceTemplatesUpdateSalesRequestVatScheme>,
    vat_country_code: Option<String>,
    agreement_id: Option<String>,
    operation_type_id: Option<String>,
    document_series_id: Option<String>,
    discount_percent: Option<String>,
    issued_by_name: Option<String>,
    issued_by_title: Option<String>,
    received_by_name: Option<String>,
    received_by_title: Option<String>,
    notes: Option<String>,
    supply_type: Option<InvoiceTemplatesUpdateSalesRequestSupplyType>,
    series_label: Option<String>,
    order_number: Option<String>,
    intrastat_transport_mode: Option<String>,
    intrastat_delivery_terms: Option<String>,
    intrastat_region: Option<String>,
    intrastat_nature_of_transaction: Option<String>,
    lines: Option<Vec<InvoiceTemplatesUpdateSalesRequestLinesItem>>,
}

impl InvoiceTemplatesUpdateSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: InvoiceTemplatesUpdateSalesRequestDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn payment_term_days(mut self, value: i64) -> Self {
        self.payment_term_days = Some(value);
        self
    }

    pub fn vat_scheme(mut self, value: InvoiceTemplatesUpdateSalesRequestVatScheme) -> Self {
        self.vat_scheme = Some(value);
        self
    }

    pub fn vat_country_code(mut self, value: impl Into<String>) -> Self {
        self.vat_country_code = Some(value.into());
        self
    }

    pub fn agreement_id(mut self, value: impl Into<String>) -> Self {
        self.agreement_id = Some(value.into());
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

    pub fn discount_percent(mut self, value: impl Into<String>) -> Self {
        self.discount_percent = Some(value.into());
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

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn supply_type(mut self, value: InvoiceTemplatesUpdateSalesRequestSupplyType) -> Self {
        self.supply_type = Some(value);
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

    pub fn lines(mut self, value: Vec<InvoiceTemplatesUpdateSalesRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceTemplatesUpdateSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceTemplatesUpdateSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoiceTemplatesUpdateSalesRequest, BuildError> {
        Ok(InvoiceTemplatesUpdateSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            document_type: self.document_type,
            partner_id: self.partner_id,
            currency: self.currency,
            payment_term_days: self.payment_term_days,
            vat_scheme: self.vat_scheme,
            vat_country_code: self.vat_country_code,
            agreement_id: self.agreement_id,
            operation_type_id: self.operation_type_id,
            document_series_id: self.document_series_id,
            discount_percent: self.discount_percent,
            issued_by_name: self.issued_by_name,
            issued_by_title: self.issued_by_title,
            received_by_name: self.received_by_name,
            received_by_title: self.received_by_title,
            notes: self.notes,
            supply_type: self.supply_type,
            series_label: self.series_label,
            order_number: self.order_number,
            intrastat_transport_mode: self.intrastat_transport_mode,
            intrastat_delivery_terms: self.intrastat_delivery_terms,
            intrastat_region: self.intrastat_region,
            intrastat_nature_of_transaction: self.intrastat_nature_of_transaction,
            lines: self.lines,
        })
    }
}
