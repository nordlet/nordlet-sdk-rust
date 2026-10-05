pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementsImportBankResponse {
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub skipped: i64,
    #[serde(default)]
    pub posted: i64,
    #[serde(rename = "customersCreated")]
    #[serde(default)]
    pub customers_created: i64,
    #[serde(rename = "invoicesCreated")]
    #[serde(default)]
    pub invoices_created: i64,
    #[serde(rename = "invoicesLinked")]
    #[serde(default)]
    pub invoices_linked: i64,
    #[serde(rename = "creditNotesCreated")]
    #[serde(default)]
    pub credit_notes_created: i64,
    #[serde(rename = "authorizationsRecorded")]
    #[serde(default)]
    pub authorizations_recorded: i64,
    #[serde(rename = "payoutsPosted")]
    #[serde(default)]
    pub payouts_posted: i64,
    #[serde(rename = "commissionsPosted")]
    #[serde(default)]
    pub commissions_posted: i64,
    #[serde(rename = "paymentsMatched")]
    #[serde(default)]
    pub payments_matched: i64,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub statements: Vec<StatementsImportBankResponseStatementsItem>,
}

impl StatementsImportBankResponse {
    pub fn builder() -> StatementsImportBankResponseBuilder {
        <StatementsImportBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementsImportBankResponseBuilder {
    imported: Option<i64>,
    skipped: Option<i64>,
    posted: Option<i64>,
    customers_created: Option<i64>,
    invoices_created: Option<i64>,
    invoices_linked: Option<i64>,
    credit_notes_created: Option<i64>,
    authorizations_recorded: Option<i64>,
    payouts_posted: Option<i64>,
    commissions_posted: Option<i64>,
    payments_matched: Option<i64>,
    warnings: Option<Vec<String>>,
    statements: Option<Vec<StatementsImportBankResponseStatementsItem>>,
}

impl StatementsImportBankResponseBuilder {
    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn posted(mut self, value: i64) -> Self {
        self.posted = Some(value);
        self
    }

    pub fn customers_created(mut self, value: i64) -> Self {
        self.customers_created = Some(value);
        self
    }

    pub fn invoices_created(mut self, value: i64) -> Self {
        self.invoices_created = Some(value);
        self
    }

    pub fn invoices_linked(mut self, value: i64) -> Self {
        self.invoices_linked = Some(value);
        self
    }

    pub fn credit_notes_created(mut self, value: i64) -> Self {
        self.credit_notes_created = Some(value);
        self
    }

    pub fn authorizations_recorded(mut self, value: i64) -> Self {
        self.authorizations_recorded = Some(value);
        self
    }

    pub fn payouts_posted(mut self, value: i64) -> Self {
        self.payouts_posted = Some(value);
        self
    }

    pub fn commissions_posted(mut self, value: i64) -> Self {
        self.commissions_posted = Some(value);
        self
    }

    pub fn payments_matched(mut self, value: i64) -> Self {
        self.payments_matched = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn statements(mut self, value: Vec<StatementsImportBankResponseStatementsItem>) -> Self {
        self.statements = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementsImportBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`imported`](StatementsImportBankResponseBuilder::imported)
    /// - [`skipped`](StatementsImportBankResponseBuilder::skipped)
    /// - [`posted`](StatementsImportBankResponseBuilder::posted)
    /// - [`customers_created`](StatementsImportBankResponseBuilder::customers_created)
    /// - [`invoices_created`](StatementsImportBankResponseBuilder::invoices_created)
    /// - [`invoices_linked`](StatementsImportBankResponseBuilder::invoices_linked)
    /// - [`credit_notes_created`](StatementsImportBankResponseBuilder::credit_notes_created)
    /// - [`authorizations_recorded`](StatementsImportBankResponseBuilder::authorizations_recorded)
    /// - [`payouts_posted`](StatementsImportBankResponseBuilder::payouts_posted)
    /// - [`commissions_posted`](StatementsImportBankResponseBuilder::commissions_posted)
    /// - [`payments_matched`](StatementsImportBankResponseBuilder::payments_matched)
    /// - [`warnings`](StatementsImportBankResponseBuilder::warnings)
    /// - [`statements`](StatementsImportBankResponseBuilder::statements)
    pub fn build(self) -> Result<StatementsImportBankResponse, BuildError> {
        Ok(StatementsImportBankResponse {
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
            posted: self
                .posted
                .ok_or_else(|| BuildError::missing_field("posted"))?,
            customers_created: self
                .customers_created
                .ok_or_else(|| BuildError::missing_field("customers_created"))?,
            invoices_created: self
                .invoices_created
                .ok_or_else(|| BuildError::missing_field("invoices_created"))?,
            invoices_linked: self
                .invoices_linked
                .ok_or_else(|| BuildError::missing_field("invoices_linked"))?,
            credit_notes_created: self
                .credit_notes_created
                .ok_or_else(|| BuildError::missing_field("credit_notes_created"))?,
            authorizations_recorded: self
                .authorizations_recorded
                .ok_or_else(|| BuildError::missing_field("authorizations_recorded"))?,
            payouts_posted: self
                .payouts_posted
                .ok_or_else(|| BuildError::missing_field("payouts_posted"))?,
            commissions_posted: self
                .commissions_posted
                .ok_or_else(|| BuildError::missing_field("commissions_posted"))?,
            payments_matched: self
                .payments_matched
                .ok_or_else(|| BuildError::missing_field("payments_matched"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            statements: self
                .statements
                .ok_or_else(|| BuildError::missing_field("statements"))?,
        })
    }
}
