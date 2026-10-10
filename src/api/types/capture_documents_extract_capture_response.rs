pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DocumentsExtractCaptureResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    pub size_bytes: i64,
    pub status: DocumentsExtractCaptureResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "pagesProcessed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_processed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction: Option<DocumentsExtractCaptureResponseExtraction>,
    #[serde(rename = "matchedPartnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_partner_id: Option<String>,
    #[serde(rename = "purchaseInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "senderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(rename = "rawText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_text: Option<String>,
}

impl DocumentsExtractCaptureResponse {
    pub fn builder() -> DocumentsExtractCaptureResponseBuilder {
        <DocumentsExtractCaptureResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsExtractCaptureResponseBuilder {
    id: Option<String>,
    file_id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    status: Option<DocumentsExtractCaptureResponseStatus>,
    provider: Option<String>,
    model: Option<String>,
    pages_processed: Option<i64>,
    extraction: Option<DocumentsExtractCaptureResponseExtraction>,
    matched_partner_id: Option<String>,
    purchase_invoice_id: Option<String>,
    error: Option<String>,
    sender_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    raw_text: Option<String>,
}

impl DocumentsExtractCaptureResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn status(mut self, value: DocumentsExtractCaptureResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn pages_processed(mut self, value: i64) -> Self {
        self.pages_processed = Some(value);
        self
    }

    pub fn extraction(mut self, value: DocumentsExtractCaptureResponseExtraction) -> Self {
        self.extraction = Some(value);
        self
    }

    pub fn matched_partner_id(mut self, value: impl Into<String>) -> Self {
        self.matched_partner_id = Some(value.into());
        self
    }

    pub fn purchase_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_invoice_id = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn sender_id(mut self, value: impl Into<String>) -> Self {
        self.sender_id = Some(value.into());
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

    pub fn raw_text(mut self, value: impl Into<String>) -> Self {
        self.raw_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentsExtractCaptureResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DocumentsExtractCaptureResponseBuilder::id)
    /// - [`file_id`](DocumentsExtractCaptureResponseBuilder::file_id)
    /// - [`file_name`](DocumentsExtractCaptureResponseBuilder::file_name)
    /// - [`mime_type`](DocumentsExtractCaptureResponseBuilder::mime_type)
    /// - [`size_bytes`](DocumentsExtractCaptureResponseBuilder::size_bytes)
    /// - [`status`](DocumentsExtractCaptureResponseBuilder::status)
    /// - [`created_at`](DocumentsExtractCaptureResponseBuilder::created_at)
    /// - [`updated_at`](DocumentsExtractCaptureResponseBuilder::updated_at)
    pub fn build(self) -> Result<DocumentsExtractCaptureResponse, BuildError> {
        Ok(DocumentsExtractCaptureResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
            size_bytes: self
                .size_bytes
                .ok_or_else(|| BuildError::missing_field("size_bytes"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            provider: self.provider,
            model: self.model,
            pages_processed: self.pages_processed,
            extraction: self.extraction,
            matched_partner_id: self.matched_partner_id,
            purchase_invoice_id: self.purchase_invoice_id,
            error: self.error,
            sender_id: self.sender_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            raw_text: self.raw_text,
        })
    }
}
