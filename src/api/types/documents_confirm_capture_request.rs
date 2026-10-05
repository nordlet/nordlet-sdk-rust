pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentsConfirmCaptureRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "newSupplier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_supplier: Option<DocumentsConfirmCaptureRequestNewSupplier>,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: NaiveDate,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<DocumentsConfirmCaptureRequestLinesItem>,
}

impl DocumentsConfirmCaptureRequest {
    pub fn builder() -> DocumentsConfirmCaptureRequestBuilder {
        <DocumentsConfirmCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsConfirmCaptureRequestBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    new_supplier: Option<DocumentsConfirmCaptureRequestNewSupplier>,
    document_number: Option<String>,
    document_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    currency: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<DocumentsConfirmCaptureRequestLinesItem>>,
}

impl DocumentsConfirmCaptureRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn new_supplier(mut self, value: DocumentsConfirmCaptureRequestNewSupplier) -> Self {
        self.new_supplier = Some(value);
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

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<DocumentsConfirmCaptureRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentsConfirmCaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DocumentsConfirmCaptureRequestBuilder::id)
    /// - [`document_number`](DocumentsConfirmCaptureRequestBuilder::document_number)
    /// - [`document_date`](DocumentsConfirmCaptureRequestBuilder::document_date)
    /// - [`lines`](DocumentsConfirmCaptureRequestBuilder::lines)
    pub fn build(self) -> Result<DocumentsConfirmCaptureRequest, BuildError> {
        Ok(DocumentsConfirmCaptureRequest {
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
