pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1PurchasesInvoicesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "documentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(rename = "documentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_date: Option<String>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<PostV1PurchasesInvoicesUpdateRequestLinesItem>>,
}

impl PostV1PurchasesInvoicesUpdateRequest {
    pub fn builder() -> PostV1PurchasesInvoicesUpdateRequestBuilder {
        <PostV1PurchasesInvoicesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesUpdateRequestBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    document_number: Option<String>,
    document_date: Option<String>,
    due_date: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<PostV1PurchasesInvoicesUpdateRequestLinesItem>>,
}

impl PostV1PurchasesInvoicesUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
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

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1PurchasesInvoicesUpdateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesInvoicesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesUpdateRequest, BuildError> {
        Ok(PostV1PurchasesInvoicesUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self.partner_id,
            document_number: self.document_number,
            document_date: self.document_date,
            due_date: self.due_date,
            currency: self.currency,
            notes: self.notes,
            lines: self.lines,
        })
    }
}
