pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1CaptureDocumentsConfirmRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "newSupplier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_supplier: Option<PostV1CaptureDocumentsConfirmRequestNewSupplier>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1CaptureDocumentsConfirmRequestLinesItem>,
}

impl PostV1CaptureDocumentsConfirmRequest {
    pub fn builder() -> PostV1CaptureDocumentsConfirmRequestBuilder {
        <PostV1CaptureDocumentsConfirmRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsConfirmRequestBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    new_supplier: Option<PostV1CaptureDocumentsConfirmRequestNewSupplier>,
    document_number: Option<String>,
    document_date: Option<String>,
    due_date: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<PostV1CaptureDocumentsConfirmRequestLinesItem>>,
}

impl PostV1CaptureDocumentsConfirmRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn new_supplier(mut self, value: PostV1CaptureDocumentsConfirmRequestNewSupplier) -> Self {
        self.new_supplier = Some(value);
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

    pub fn lines(mut self, value: Vec<PostV1CaptureDocumentsConfirmRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsConfirmRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CaptureDocumentsConfirmRequestBuilder::id)
    /// - [`document_number`](PostV1CaptureDocumentsConfirmRequestBuilder::document_number)
    /// - [`document_date`](PostV1CaptureDocumentsConfirmRequestBuilder::document_date)
    /// - [`lines`](PostV1CaptureDocumentsConfirmRequestBuilder::lines)
    pub fn build(self) -> Result<PostV1CaptureDocumentsConfirmRequest, BuildError> {
        Ok(PostV1CaptureDocumentsConfirmRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self.partner_id,
            new_supplier: self.new_supplier,
            document_number: self
                .document_number
                .ok_or_else(|| BuildError::missing_field("document_number"))?,
            document_date: self
                .document_date
                .ok_or_else(|| BuildError::missing_field("document_date"))?,
            due_date: self.due_date,
            currency: self.currency,
            notes: self.notes,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
