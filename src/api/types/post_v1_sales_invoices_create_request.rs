pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1SalesInvoicesCreateRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1SalesInvoicesCreateRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "creditedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_invoice_id: Option<String>,
    #[serde(rename = "vatScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_scheme: Option<PostV1SalesInvoicesCreateRequestVatScheme>,
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "deemedSupplier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deemed_supplier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1SalesInvoicesCreateRequestLinesItem>,
}

impl PostV1SalesInvoicesCreateRequest {
    pub fn builder() -> PostV1SalesInvoicesCreateRequestBuilder {
        <PostV1SalesInvoicesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesCreateRequestBuilder {
    partner_id: Option<String>,
    r#type: Option<PostV1SalesInvoicesCreateRequestType>,
    currency: Option<String>,
    issue_date: Option<String>,
    due_date: Option<String>,
    credited_invoice_id: Option<String>,
    vat_scheme: Option<PostV1SalesInvoicesCreateRequestVatScheme>,
    vat_country_code: Option<String>,
    deemed_supplier: Option<bool>,
    notes: Option<String>,
    lines: Option<Vec<PostV1SalesInvoicesCreateRequestLinesItem>>,
}

impl PostV1SalesInvoicesCreateRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1SalesInvoicesCreateRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn credited_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.credited_invoice_id = Some(value.into());
        self
    }

    pub fn vat_scheme(mut self, value: PostV1SalesInvoicesCreateRequestVatScheme) -> Self {
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

    pub fn lines(mut self, value: Vec<PostV1SalesInvoicesCreateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1SalesInvoicesCreateRequestBuilder::partner_id)
    /// - [`lines`](PostV1SalesInvoicesCreateRequestBuilder::lines)
    pub fn build(self) -> Result<PostV1SalesInvoicesCreateRequest, BuildError> {
        Ok(PostV1SalesInvoicesCreateRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            r#type: self.r#type,
            currency: self.currency,
            issue_date: self.issue_date,
            due_date: self.due_date,
            credited_invoice_id: self.credited_invoice_id,
            vat_scheme: self.vat_scheme,
            vat_country_code: self.vat_country_code,
            deemed_supplier: self.deemed_supplier,
            notes: self.notes,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
