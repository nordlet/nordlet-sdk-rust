pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    pub r#type: PostV1PurchasesInvoicesGetResponseType,
    pub status: PostV1PurchasesInvoicesGetResponseStatus,
    #[serde(rename = "paymentStatus")]
    pub payment_status: PostV1PurchasesInvoicesGetResponsePaymentStatus,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: String,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "registrationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub lines: Vec<PostV1PurchasesInvoicesGetResponseLinesItem>,
}

impl PostV1PurchasesInvoicesGetResponse {
    pub fn builder() -> PostV1PurchasesInvoicesGetResponseBuilder {
        <PostV1PurchasesInvoicesGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesGetResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    r#type: Option<PostV1PurchasesInvoicesGetResponseType>,
    status: Option<PostV1PurchasesInvoicesGetResponseStatus>,
    payment_status: Option<PostV1PurchasesInvoicesGetResponsePaymentStatus>,
    document_number: Option<String>,
    document_date: Option<String>,
    due_date: Option<String>,
    registration_date: Option<String>,
    currency: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    paid_amount: Option<String>,
    journal_transaction_id: Option<String>,
    credited_invoice_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    lines: Option<Vec<PostV1PurchasesInvoicesGetResponseLinesItem>>,
}

impl PostV1PurchasesInvoicesGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1PurchasesInvoicesGetResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1PurchasesInvoicesGetResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_status(
        mut self,
        value: PostV1PurchasesInvoicesGetResponsePaymentStatus,
    ) -> Self {
        self.payment_status = Some(value);
        self
    }

    pub fn document_number(mut self, value: impl Into<String>) -> Self {
        self.document_number = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: impl Into<String>) -> Self {
        self.document_date = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
        self
    }

    pub fn registration_date(mut self, value: impl Into<String>) -> Self {
        self.registration_date = Some(value.into());
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

    pub fn lines(mut self, value: Vec<PostV1PurchasesInvoicesGetResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesInvoicesGetResponseBuilder::id)
    /// - [`partner_id`](PostV1PurchasesInvoicesGetResponseBuilder::partner_id)
    /// - [`r#type`](PostV1PurchasesInvoicesGetResponseBuilder::r#type)
    /// - [`status`](PostV1PurchasesInvoicesGetResponseBuilder::status)
    /// - [`payment_status`](PostV1PurchasesInvoicesGetResponseBuilder::payment_status)
    /// - [`document_number`](PostV1PurchasesInvoicesGetResponseBuilder::document_number)
    /// - [`document_date`](PostV1PurchasesInvoicesGetResponseBuilder::document_date)
    /// - [`currency`](PostV1PurchasesInvoicesGetResponseBuilder::currency)
    /// - [`net_total`](PostV1PurchasesInvoicesGetResponseBuilder::net_total)
    /// - [`vat_total`](PostV1PurchasesInvoicesGetResponseBuilder::vat_total)
    /// - [`gross_total`](PostV1PurchasesInvoicesGetResponseBuilder::gross_total)
    /// - [`paid_amount`](PostV1PurchasesInvoicesGetResponseBuilder::paid_amount)
    /// - [`created_at`](PostV1PurchasesInvoicesGetResponseBuilder::created_at)
    /// - [`updated_at`](PostV1PurchasesInvoicesGetResponseBuilder::updated_at)
    /// - [`lines`](PostV1PurchasesInvoicesGetResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesGetResponse, BuildError> {
        Ok(PostV1PurchasesInvoicesGetResponse {
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
