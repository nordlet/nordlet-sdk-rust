pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsConfirmResponseCaptureExtraction {
    #[serde(default)]
    pub supplier: PostV1CaptureDocumentsConfirmResponseCaptureExtractionSupplier,
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
    #[serde(rename = "netTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total: Option<String>,
    #[serde(rename = "vatTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_total: Option<String>,
    #[serde(rename = "grossTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_total: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1CaptureDocumentsConfirmResponseCaptureExtractionLinesItem>,
}

impl PostV1CaptureDocumentsConfirmResponseCaptureExtraction {
    pub fn builder() -> PostV1CaptureDocumentsConfirmResponseCaptureExtractionBuilder {
        <PostV1CaptureDocumentsConfirmResponseCaptureExtractionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsConfirmResponseCaptureExtractionBuilder {
    supplier: Option<PostV1CaptureDocumentsConfirmResponseCaptureExtractionSupplier>,
    document_number: Option<String>,
    document_date: Option<String>,
    due_date: Option<String>,
    currency: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<PostV1CaptureDocumentsConfirmResponseCaptureExtractionLinesItem>>,
}

impl PostV1CaptureDocumentsConfirmResponseCaptureExtractionBuilder {
    pub fn supplier(
        mut self,
        value: PostV1CaptureDocumentsConfirmResponseCaptureExtractionSupplier,
    ) -> Self {
        self.supplier = Some(value);
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

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn lines(
        mut self,
        value: Vec<PostV1CaptureDocumentsConfirmResponseCaptureExtractionLinesItem>,
    ) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsConfirmResponseCaptureExtraction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`supplier`](PostV1CaptureDocumentsConfirmResponseCaptureExtractionBuilder::supplier)
    /// - [`lines`](PostV1CaptureDocumentsConfirmResponseCaptureExtractionBuilder::lines)
    pub fn build(
        self,
    ) -> Result<PostV1CaptureDocumentsConfirmResponseCaptureExtraction, BuildError> {
        Ok(PostV1CaptureDocumentsConfirmResponseCaptureExtraction {
            supplier: self
                .supplier
                .ok_or_else(|| BuildError::missing_field("supplier"))?,
            document_number: self.document_number,
            document_date: self.document_date,
            due_date: self.due_date,
            currency: self.currency,
            net_total: self.net_total,
            vat_total: self.vat_total,
            gross_total: self.gross_total,
            notes: self.notes,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
