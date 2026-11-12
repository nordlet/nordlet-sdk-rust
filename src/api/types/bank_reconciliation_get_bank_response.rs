pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReconciliationGetBankResponse {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "statementBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_balance: Option<String>,
    #[serde(rename = "statementBalanceSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_balance_source: Option<ReconciliationGetBankResponseStatementBalanceSource>,
    #[serde(rename = "statementClosingDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_closing_date: Option<NaiveDate>,
    #[serde(rename = "ledgerBalance")]
    #[serde(default)]
    pub ledger_balance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference: Option<String>,
    #[serde(rename = "unbookedCount")]
    #[serde(default)]
    pub unbooked_count: i64,
    #[serde(rename = "unbookedTotal")]
    #[serde(default)]
    pub unbooked_total: String,
    #[serde(rename = "unbookedLines")]
    #[serde(default)]
    pub unbooked_lines: Vec<ReconciliationGetBankResponseUnbookedLinesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl ReconciliationGetBankResponse {
    pub fn builder() -> ReconciliationGetBankResponseBuilder {
        <ReconciliationGetBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReconciliationGetBankResponseBuilder {
    bank_account_id: Option<String>,
    account_code: Option<String>,
    currency: Option<String>,
    date: Option<NaiveDate>,
    statement_balance: Option<String>,
    statement_balance_source: Option<ReconciliationGetBankResponseStatementBalanceSource>,
    statement_closing_date: Option<NaiveDate>,
    ledger_balance: Option<String>,
    difference: Option<String>,
    unbooked_count: Option<i64>,
    unbooked_total: Option<String>,
    unbooked_lines: Option<Vec<ReconciliationGetBankResponseUnbookedLinesItem>>,
    warnings: Option<Vec<String>>,
}

impl ReconciliationGetBankResponseBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn statement_balance(mut self, value: impl Into<String>) -> Self {
        self.statement_balance = Some(value.into());
        self
    }

    pub fn statement_balance_source(
        mut self,
        value: ReconciliationGetBankResponseStatementBalanceSource,
    ) -> Self {
        self.statement_balance_source = Some(value);
        self
    }

    pub fn statement_closing_date(mut self, value: NaiveDate) -> Self {
        self.statement_closing_date = Some(value);
        self
    }

    pub fn ledger_balance(mut self, value: impl Into<String>) -> Self {
        self.ledger_balance = Some(value.into());
        self
    }

    pub fn difference(mut self, value: impl Into<String>) -> Self {
        self.difference = Some(value.into());
        self
    }

    pub fn unbooked_count(mut self, value: i64) -> Self {
        self.unbooked_count = Some(value);
        self
    }

    pub fn unbooked_total(mut self, value: impl Into<String>) -> Self {
        self.unbooked_total = Some(value.into());
        self
    }

    pub fn unbooked_lines(
        mut self,
        value: Vec<ReconciliationGetBankResponseUnbookedLinesItem>,
    ) -> Self {
        self.unbooked_lines = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReconciliationGetBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](ReconciliationGetBankResponseBuilder::bank_account_id)
    /// - [`account_code`](ReconciliationGetBankResponseBuilder::account_code)
    /// - [`currency`](ReconciliationGetBankResponseBuilder::currency)
    /// - [`date`](ReconciliationGetBankResponseBuilder::date)
    /// - [`ledger_balance`](ReconciliationGetBankResponseBuilder::ledger_balance)
    /// - [`unbooked_count`](ReconciliationGetBankResponseBuilder::unbooked_count)
    /// - [`unbooked_total`](ReconciliationGetBankResponseBuilder::unbooked_total)
    /// - [`unbooked_lines`](ReconciliationGetBankResponseBuilder::unbooked_lines)
    /// - [`warnings`](ReconciliationGetBankResponseBuilder::warnings)
    pub fn build(self) -> Result<ReconciliationGetBankResponse, BuildError> {
        Ok(ReconciliationGetBankResponse {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            statement_balance: self.statement_balance,
            statement_balance_source: self.statement_balance_source,
            statement_closing_date: self.statement_closing_date,
            ledger_balance: self
                .ledger_balance
                .ok_or_else(|| BuildError::missing_field("ledger_balance"))?,
            difference: self.difference,
            unbooked_count: self
                .unbooked_count
                .ok_or_else(|| BuildError::missing_field("unbooked_count"))?,
            unbooked_total: self
                .unbooked_total
                .ok_or_else(|| BuildError::missing_field("unbooked_total"))?,
            unbooked_lines: self
                .unbooked_lines
                .ok_or_else(|| BuildError::missing_field("unbooked_lines"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
