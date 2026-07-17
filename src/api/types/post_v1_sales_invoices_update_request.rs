pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1SalesInvoicesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "vatScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_scheme: Option<PostV1SalesInvoicesUpdateRequestVatScheme>,
    #[serde(rename = "vatCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_country_code: Option<String>,
    #[serde(rename = "deemedSupplier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deemed_supplier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<PostV1SalesInvoicesUpdateRequestLinesItem>>,
}

impl PostV1SalesInvoicesUpdateRequest {
    pub fn builder() -> PostV1SalesInvoicesUpdateRequestBuilder {
        <PostV1SalesInvoicesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUpdateRequestBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    currency: Option<String>,
    issue_date: Option<String>,
    due_date: Option<String>,
    vat_scheme: Option<PostV1SalesInvoicesUpdateRequestVatScheme>,
    vat_country_code: Option<String>,
    deemed_supplier: Option<bool>,
    notes: Option<String>,
    lines: Option<Vec<PostV1SalesInvoicesUpdateRequestLinesItem>>,
}

impl PostV1SalesInvoicesUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
        self
    }

    pub fn vat_scheme(mut self, value: PostV1SalesInvoicesUpdateRequestVatScheme) -> Self {
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

    pub fn lines(mut self, value: Vec<PostV1SalesInvoicesUpdateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesUpdateRequest, BuildError> {
        Ok(PostV1SalesInvoicesUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self.partner_id,
            currency: self.currency,
            issue_date: self.issue_date,
            due_date: self.due_date,
            vat_scheme: self.vat_scheme,
            vat_country_code: self.vat_country_code,
            deemed_supplier: self.deemed_supplier,
            notes: self.notes,
            lines: self.lines,
        })
    }
}
