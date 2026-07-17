pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1PurchasesInvoicesCreateRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1PurchasesInvoicesCreateRequestType>,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: String,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "creditedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1PurchasesInvoicesCreateRequestLinesItem>,
}

impl PostV1PurchasesInvoicesCreateRequest {
    pub fn builder() -> PostV1PurchasesInvoicesCreateRequestBuilder {
        <PostV1PurchasesInvoicesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesCreateRequestBuilder {
    partner_id: Option<String>,
    r#type: Option<PostV1PurchasesInvoicesCreateRequestType>,
    document_number: Option<String>,
    document_date: Option<String>,
    due_date: Option<String>,
    currency: Option<String>,
    credited_invoice_id: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<PostV1PurchasesInvoicesCreateRequestLinesItem>>,
}

impl PostV1PurchasesInvoicesCreateRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1PurchasesInvoicesCreateRequestType) -> Self {
        self.r#type = Some(value);
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

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn lines(mut self, value: Vec<PostV1PurchasesInvoicesCreateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1PurchasesInvoicesCreateRequestBuilder::partner_id)
    /// - [`document_number`](PostV1PurchasesInvoicesCreateRequestBuilder::document_number)
    /// - [`document_date`](PostV1PurchasesInvoicesCreateRequestBuilder::document_date)
    /// - [`lines`](PostV1PurchasesInvoicesCreateRequestBuilder::lines)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesCreateRequest, BuildError> {
        Ok(PostV1PurchasesInvoicesCreateRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            r#type: self.r#type,
            document_number: self
                .document_number
                .ok_or_else(|| BuildError::missing_field("document_number"))?,
            document_date: self
                .document_date
                .ok_or_else(|| BuildError::missing_field("document_date"))?,
            due_date: self.due_date,
            currency: self.currency,
            credited_invoice_id: self.credited_invoice_id,
            notes: self.notes,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
