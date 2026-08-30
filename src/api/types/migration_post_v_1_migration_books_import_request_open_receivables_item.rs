pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportRequestOpenReceivablesItem {
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
    #[serde(default)]
    pub number: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
}

impl PostV1MigrationBooksImportRequestOpenReceivablesItem {
    pub fn builder() -> PostV1MigrationBooksImportRequestOpenReceivablesItemBuilder {
        <PostV1MigrationBooksImportRequestOpenReceivablesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportRequestOpenReceivablesItemBuilder {
    partner_code: Option<String>,
    due_date: Option<String>,
    currency: Option<String>,
    gross_total: Option<String>,
    vat_total: Option<String>,
    outstanding: Option<String>,
    fx_rate: Option<String>,
    notes: Option<String>,
    number: Option<String>,
    issue_date: Option<String>,
}

impl PostV1MigrationBooksImportRequestOpenReceivablesItemBuilder {
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

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportRequestOpenReceivablesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_code`](PostV1MigrationBooksImportRequestOpenReceivablesItemBuilder::partner_code)
    /// - [`gross_total`](PostV1MigrationBooksImportRequestOpenReceivablesItemBuilder::gross_total)
    /// - [`number`](PostV1MigrationBooksImportRequestOpenReceivablesItemBuilder::number)
    /// - [`issue_date`](PostV1MigrationBooksImportRequestOpenReceivablesItemBuilder::issue_date)
    pub fn build(self) -> Result<PostV1MigrationBooksImportRequestOpenReceivablesItem, BuildError> {
        Ok(PostV1MigrationBooksImportRequestOpenReceivablesItem {
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
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            issue_date: self
                .issue_date
                .ok_or_else(|| BuildError::missing_field("issue_date"))?,
        })
    }
}
