pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicesCreatePurchasesResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    pub r#type: InvoicesCreatePurchasesResponseType,
    pub status: InvoicesCreatePurchasesResponseStatus,
    #[serde(rename = "paymentStatus")]
    pub payment_status: InvoicesCreatePurchasesResponsePaymentStatus,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: NaiveDate,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(rename = "registrationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<NaiveDate>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "paidAmount")]
    #[serde(default)]
    pub paid_amount: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(rename = "creditedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_id: Option<String>,
    #[serde(rename = "purchaseOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_id: Option<String>,
    #[serde(rename = "operationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
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
    #[serde(rename = "einvoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub einvoice_number: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub lines: Vec<InvoicesCreatePurchasesResponseLinesItem>,
}

impl InvoicesCreatePurchasesResponse {
    pub fn builder() -> InvoicesCreatePurchasesResponseBuilder {
        <InvoicesCreatePurchasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCreatePurchasesResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    r#type: Option<InvoicesCreatePurchasesResponseType>,
    status: Option<InvoicesCreatePurchasesResponseStatus>,
    payment_status: Option<InvoicesCreatePurchasesResponsePaymentStatus>,
    document_number: Option<String>,
    document_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    registration_date: Option<NaiveDate>,
    currency: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    paid_amount: Option<String>,
    journal_transaction_id: Option<String>,
    credited_invoice_id: Option<String>,
    purchase_order_id: Option<String>,
    operation_type_id: Option<String>,
    notes: Option<String>,
    intrastat_transport_mode: Option<String>,
    intrastat_delivery_terms: Option<String>,
    intrastat_region: Option<String>,
    intrastat_nature_of_transaction: Option<String>,
    einvoice_number: Option<String>,
    document_ref: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    lines: Option<Vec<InvoicesCreatePurchasesResponseLinesItem>>,
}

impl InvoicesCreatePurchasesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: InvoicesCreatePurchasesResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: InvoicesCreatePurchasesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_status(mut self, value: InvoicesCreatePurchasesResponsePaymentStatus) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn document_number(mut self, value: impl Into<String>) -> Self {
        self.document_number = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: NaiveDate) -> Self {
        self.document_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn registration_date(mut self, value: NaiveDate) -> Self {
        self.registration_date = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn paid_amount(mut self, value: impl Into<String>) -> Self {
        self.paid_amount = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn credited_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.credited_invoice_id = Some(value.into());
        self
    }

    pub fn purchase_order_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_order_id = Some(value.into());
        self
    }

    pub fn operation_type_id(mut self, value: impl Into<String>) -> Self {
        self.operation_type_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
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

    pub fn einvoice_number(mut self, value: impl Into<String>) -> Self {
        self.einvoice_number = Some(value.into());
        self
    }

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<InvoicesCreatePurchasesResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCreatePurchasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesCreatePurchasesResponseBuilder::id)
    /// - [`partner_id`](InvoicesCreatePurchasesResponseBuilder::partner_id)
    /// - [`r#type`](InvoicesCreatePurchasesResponseBuilder::r#type)
    /// - [`status`](InvoicesCreatePurchasesResponseBuilder::status)
    /// - [`payment_status`](InvoicesCreatePurchasesResponseBuilder::payment_status)
    /// - [`document_number`](InvoicesCreatePurchasesResponseBuilder::document_number)
    /// - [`document_date`](InvoicesCreatePurchasesResponseBuilder::document_date)
    /// - [`currency`](InvoicesCreatePurchasesResponseBuilder::currency)
    /// - [`net_total`](InvoicesCreatePurchasesResponseBuilder::net_total)
    /// - [`vat_total`](InvoicesCreatePurchasesResponseBuilder::vat_total)
    /// - [`gross_total`](InvoicesCreatePurchasesResponseBuilder::gross_total)
    /// - [`paid_amount`](InvoicesCreatePurchasesResponseBuilder::paid_amount)
    /// - [`created_at`](InvoicesCreatePurchasesResponseBuilder::created_at)
    /// - [`updated_at`](InvoicesCreatePurchasesResponseBuilder::updated_at)
    /// - [`lines`](InvoicesCreatePurchasesResponseBuilder::lines)
    pub fn build(self) -> Result<InvoicesCreatePurchasesResponse, BuildError> {
        Ok(InvoicesCreatePurchasesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            payment_status: self
                .payment_status
                .ok_or_else(|| BuildError::missing_field("payment_status"))?,
            document_number: self
                .document_number
                .ok_or_else(|| BuildError::missing_field("document_number"))?,
            document_date: self
                .document_date
                .ok_or_else(|| BuildError::missing_field("document_date"))?,
            due_date: self.due_date,
            registration_date: self.registration_date,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            paid_amount: self
                .paid_amount
                .ok_or_else(|| BuildError::missing_field("paid_amount"))?,
            journal_transaction_id: self.journal_transaction_id,
            credited_invoice_id: self.credited_invoice_id,
            purchase_order_id: self.purchase_order_id,
            operation_type_id: self.operation_type_id,
            notes: self.notes,
            intrastat_transport_mode: self.intrastat_transport_mode,
            intrastat_delivery_terms: self.intrastat_delivery_terms,
            intrastat_region: self.intrastat_region,
            intrastat_nature_of_transaction: self.intrastat_nature_of_transaction,
            einvoice_number: self.einvoice_number,
            document_ref: self.document_ref,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
