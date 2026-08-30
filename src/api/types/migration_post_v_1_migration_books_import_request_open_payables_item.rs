pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportRequestOpenPayablesItem {
    #[serde(rename = "partnerCode")]
    #[serde(default)]
    pub partner_code: String,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_total: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outstanding: Option<String>,
    #[serde(rename = "fxRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: String,
}

impl PostV1MigrationBooksImportRequestOpenPayablesItem {
    pub fn builder() -> PostV1MigrationBooksImportRequestOpenPayablesItemBuilder {
        <PostV1MigrationBooksImportRequestOpenPayablesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportRequestOpenPayablesItemBuilder {
    partner_code: Option<String>,
    due_date: Option<String>,
    currency: Option<String>,
    gross_total: Option<String>,
    vat_total: Option<String>,
    outstanding: Option<String>,
    fx_rate: Option<String>,
    notes: Option<String>,
    document_number: Option<String>,
    document_date: Option<String>,
}

impl PostV1MigrationBooksImportRequestOpenPayablesItemBuilder {
    pub fn partner_code(mut self, value: impl Into<String>) -> Self {
        self.partner_code = Some(value.into());
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

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn outstanding(mut self, value: impl Into<String>) -> Self {
        self.outstanding = Some(value.into());
        self
    }

    pub fn fx_rate(mut self, value: impl Into<String>) -> Self {
        self.fx_rate = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportRequestOpenPayablesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_code`](PostV1MigrationBooksImportRequestOpenPayablesItemBuilder::partner_code)
    /// - [`gross_total`](PostV1MigrationBooksImportRequestOpenPayablesItemBuilder::gross_total)
    /// - [`document_number`](PostV1MigrationBooksImportRequestOpenPayablesItemBuilder::document_number)
    /// - [`document_date`](PostV1MigrationBooksImportRequestOpenPayablesItemBuilder::document_date)
    pub fn build(self) -> Result<PostV1MigrationBooksImportRequestOpenPayablesItem, BuildError> {
        Ok(PostV1MigrationBooksImportRequestOpenPayablesItem {
            partner_code: self
                .partner_code
                .ok_or_else(|| BuildError::missing_field("partner_code"))?,
            due_date: self.due_date,
            currency: self.currency,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            vat_total: self.vat_total,
            outstanding: self.outstanding,
            fx_rate: self.fx_rate,
            notes: self.notes,
            document_number: self
                .document_number
                .ok_or_else(|| BuildError::missing_field("document_number"))?,
            document_date: self
                .document_date
                .ok_or_else(|| BuildError::missing_field("document_date"))?,
        })
    }
}
