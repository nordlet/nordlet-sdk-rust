pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentsListCaptureResponseRowsItemExtraction {
    #[serde(default)]
    pub supplier: DocumentsListCaptureResponseRowsItemExtractionSupplier,
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
    pub lines: Vec<DocumentsListCaptureResponseRowsItemExtractionLinesItem>,
}

impl DocumentsListCaptureResponseRowsItemExtraction {
    pub fn builder() -> DocumentsListCaptureResponseRowsItemExtractionBuilder {
        <DocumentsListCaptureResponseRowsItemExtractionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsListCaptureResponseRowsItemExtractionBuilder {
    supplier: Option<DocumentsListCaptureResponseRowsItemExtractionSupplier>,
    document_number: Option<String>,
    document_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    currency: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<DocumentsListCaptureResponseRowsItemExtractionLinesItem>>,
}

impl DocumentsListCaptureResponseRowsItemExtractionBuilder {
    pub fn supplier(
        mut self,
        value: DocumentsListCaptureResponseRowsItemExtractionSupplier,
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
        value: Vec<DocumentsListCaptureResponseRowsItemExtractionLinesItem>,
    ) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentsListCaptureResponseRowsItemExtraction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`supplier`](DocumentsListCaptureResponseRowsItemExtractionBuilder::supplier)
    /// - [`lines`](DocumentsListCaptureResponseRowsItemExtractionBuilder::lines)
    pub fn build(self) -> Result<DocumentsListCaptureResponseRowsItemExtraction, BuildError> {
        Ok(DocumentsListCaptureResponseRowsItemExtraction {
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
