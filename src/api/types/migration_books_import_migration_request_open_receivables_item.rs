pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationRequestOpenReceivablesItem {
    #[serde(rename = "partnerCode")]
    #[serde(default)]
    pub partner_code: String,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
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
    pub issue_date: NaiveDate,
}

impl BooksImportMigrationRequestOpenReceivablesItem {
    pub fn builder() -> BooksImportMigrationRequestOpenReceivablesItemBuilder {
        <BooksImportMigrationRequestOpenReceivablesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationRequestOpenReceivablesItemBuilder {
    partner_code: Option<String>,
    due_date: Option<NaiveDate>,
    currency: Option<String>,
    gross_total: Option<String>,
    vat_total: Option<String>,
    outstanding: Option<String>,
    fx_rate: Option<String>,
    notes: Option<String>,
    number: Option<String>,
    issue_date: Option<NaiveDate>,
}

impl BooksImportMigrationRequestOpenReceivablesItemBuilder {
    pub fn partner_code(mut self, value: impl Into<String>) -> Self {
        self.partner_code = Some(value.into());
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

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationRequestOpenReceivablesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_code`](BooksImportMigrationRequestOpenReceivablesItemBuilder::partner_code)
    /// - [`gross_total`](BooksImportMigrationRequestOpenReceivablesItemBuilder::gross_total)
    /// - [`number`](BooksImportMigrationRequestOpenReceivablesItemBuilder::number)
    /// - [`issue_date`](BooksImportMigrationRequestOpenReceivablesItemBuilder::issue_date)
    pub fn build(self) -> Result<BooksImportMigrationRequestOpenReceivablesItem, BuildError> {
        Ok(BooksImportMigrationRequestOpenReceivablesItem {
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
