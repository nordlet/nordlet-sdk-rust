pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    pub r#type: PostV1SalesInvoicesGetResponseType,
    pub status: PostV1SalesInvoicesGetResponseStatus,
    #[serde(rename = "paymentStatus")]
    pub payment_status: PostV1SalesInvoicesGetResponsePaymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(rename = "fullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_number: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
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
    #[serde(rename = "appliedToInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_to_invoice_id: Option<String>,
    #[serde(rename = "creditedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_id: Option<String>,
    #[serde(rename = "vatScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_scheme: Option<PostV1SalesInvoicesGetResponseVatScheme>,
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "deemedSupplier")]
    #[serde(default)]
    pub deemed_supplier: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub lines: Vec<PostV1SalesInvoicesGetResponseLinesItem>,
}

impl PostV1SalesInvoicesGetResponse {
    pub fn builder() -> PostV1SalesInvoicesGetResponseBuilder {
        <PostV1SalesInvoicesGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesGetResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    r#type: Option<PostV1SalesInvoicesGetResponseType>,
    status: Option<PostV1SalesInvoicesGetResponseStatus>,
    payment_status: Option<PostV1SalesInvoicesGetResponsePaymentStatus>,
    series: Option<String>,
    number: Option<i64>,
    full_number: Option<String>,
    issue_date: Option<String>,
    due_date: Option<String>,
    currency: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    paid_amount: Option<String>,
    journal_transaction_id: Option<String>,
    applied_to_invoice_id: Option<String>,
    credited_invoice_id: Option<String>,
    vat_scheme: Option<PostV1SalesInvoicesGetResponseVatScheme>,
    vat_country_code: Option<String>,
    deemed_supplier: Option<bool>,
    notes: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    lines: Option<Vec<PostV1SalesInvoicesGetResponseLinesItem>>,
}

impl PostV1SalesInvoicesGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1SalesInvoicesGetResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1SalesInvoicesGetResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_status(mut self, value: PostV1SalesInvoicesGetResponsePaymentStatus) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn number(mut self, value: i64) -> Self {
        self.number = Some(value);
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
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

    pub fn applied_to_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.applied_to_invoice_id = Some(value.into());
        self
    }

    pub fn credited_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.credited_invoice_id = Some(value.into());
        self
    }

    pub fn vat_scheme(mut self, value: PostV1SalesInvoicesGetResponseVatScheme) -> Self {
        self.vat_scheme = Some(value);
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1SalesInvoicesGetResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesGetResponseBuilder::id)
    /// - [`partner_id`](PostV1SalesInvoicesGetResponseBuilder::partner_id)
    /// - [`r#type`](PostV1SalesInvoicesGetResponseBuilder::r#type)
    /// - [`status`](PostV1SalesInvoicesGetResponseBuilder::status)
    /// - [`payment_status`](PostV1SalesInvoicesGetResponseBuilder::payment_status)
    /// - [`currency`](PostV1SalesInvoicesGetResponseBuilder::currency)
    /// - [`net_total`](PostV1SalesInvoicesGetResponseBuilder::net_total)
    /// - [`vat_total`](PostV1SalesInvoicesGetResponseBuilder::vat_total)
    /// - [`gross_total`](PostV1SalesInvoicesGetResponseBuilder::gross_total)
    /// - [`paid_amount`](PostV1SalesInvoicesGetResponseBuilder::paid_amount)
    /// - [`deemed_supplier`](PostV1SalesInvoicesGetResponseBuilder::deemed_supplier)
    /// - [`created_at`](PostV1SalesInvoicesGetResponseBuilder::created_at)
    /// - [`updated_at`](PostV1SalesInvoicesGetResponseBuilder::updated_at)
    /// - [`lines`](PostV1SalesInvoicesGetResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1SalesInvoicesGetResponse, BuildError> {
        Ok(PostV1SalesInvoicesGetResponse {
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
            series: self.series,
            number: self.number,
            full_number: self.full_number,
            issue_date: self.issue_date,
            due_date: self.due_date,
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
            applied_to_invoice_id: self.applied_to_invoice_id,
            credited_invoice_id: self.credited_invoice_id,
            vat_scheme: self.vat_scheme,
            vat_country_code: self.vat_country_code,
            deemed_supplier: self
                .deemed_supplier
                .ok_or_else(|| BuildError::missing_field("deemed_supplier"))?,
            notes: self.notes,
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
