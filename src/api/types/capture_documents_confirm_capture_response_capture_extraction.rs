pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentsConfirmCaptureResponseCaptureExtraction {
    #[serde(rename = "documentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<DocumentsConfirmCaptureResponseCaptureExtractionDocumentType>,
    #[serde(default)]
    pub supplier: DocumentsConfirmCaptureResponseCaptureExtractionSupplier,
    #[serde(rename = "documentNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(rename = "documentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_date: Option<NaiveDate>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
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
    pub lines: Vec<DocumentsConfirmCaptureResponseCaptureExtractionLinesItem>,
    #[serde(rename = "oppositeLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opposite_lines:
        Option<Vec<DocumentsConfirmCaptureResponseCaptureExtractionOppositeLinesItem>>,
}

impl DocumentsConfirmCaptureResponseCaptureExtraction {
    pub fn builder() -> DocumentsConfirmCaptureResponseCaptureExtractionBuilder {
        <DocumentsConfirmCaptureResponseCaptureExtractionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsConfirmCaptureResponseCaptureExtractionBuilder {
    document_type: Option<DocumentsConfirmCaptureResponseCaptureExtractionDocumentType>,
    supplier: Option<DocumentsConfirmCaptureResponseCaptureExtractionSupplier>,
    document_number: Option<String>,
    document_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    currency: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<DocumentsConfirmCaptureResponseCaptureExtractionLinesItem>>,
    opposite_lines: Option<Vec<DocumentsConfirmCaptureResponseCaptureExtractionOppositeLinesItem>>,
}

impl DocumentsConfirmCaptureResponseCaptureExtractionBuilder {
    pub fn document_type(
        mut self,
        value: DocumentsConfirmCaptureResponseCaptureExtractionDocumentType,
    ) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn supplier(
        mut self,
        value: DocumentsConfirmCaptureResponseCaptureExtractionSupplier,
    ) -> Self {
        self.supplier = Some(value);
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
        value: Vec<DocumentsConfirmCaptureResponseCaptureExtractionLinesItem>,
    ) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn opposite_lines(
        mut self,
        value: Vec<DocumentsConfirmCaptureResponseCaptureExtractionOppositeLinesItem>,
    ) -> Self {
        self.opposite_lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentsConfirmCaptureResponseCaptureExtraction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`supplier`](DocumentsConfirmCaptureResponseCaptureExtractionBuilder::supplier)
    /// - [`lines`](DocumentsConfirmCaptureResponseCaptureExtractionBuilder::lines)
    pub fn build(self) -> Result<DocumentsConfirmCaptureResponseCaptureExtraction, BuildError> {
        Ok(DocumentsConfirmCaptureResponseCaptureExtraction {
            document_type: self.document_type,
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
            opposite_lines: self.opposite_lines,
        })
    }
}
