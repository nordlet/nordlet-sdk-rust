pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementsImportBankResponseStatementsItem {
    #[serde(rename = "statementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "fromDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<NaiveDate>,
    #[serde(rename = "toDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<NaiveDate>,
    #[serde(rename = "openingBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balance: Option<String>,
    #[serde(rename = "closingBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_balance: Option<String>,
    #[serde(rename = "transactionCount")]
    #[serde(default)]
    pub transaction_count: i64,
}

impl StatementsImportBankResponseStatementsItem {
    pub fn builder() -> StatementsImportBankResponseStatementsItemBuilder {
        <StatementsImportBankResponseStatementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementsImportBankResponseStatementsItemBuilder {
    statement_id: Option<String>,
    iban: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    opening_balance: Option<String>,
    closing_balance: Option<String>,
    transaction_count: Option<i64>,
}

impl StatementsImportBankResponseStatementsItemBuilder {
    pub fn statement_id(mut self, value: impl Into<String>) -> Self {
        self.statement_id = Some(value.into());
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn opening_balance(mut self, value: impl Into<String>) -> Self {
        self.opening_balance = Some(value.into());
        self
    }

    pub fn closing_balance(mut self, value: impl Into<String>) -> Self {
        self.closing_balance = Some(value.into());
        self
    }

    pub fn transaction_count(mut self, value: i64) -> Self {
        self.transaction_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementsImportBankResponseStatementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_count`](StatementsImportBankResponseStatementsItemBuilder::transaction_count)
    pub fn build(self) -> Result<StatementsImportBankResponseStatementsItem, BuildError> {
        Ok(StatementsImportBankResponseStatementsItem {
            statement_id: self.statement_id,
            iban: self.iban,
            from_date: self.from_date,
            to_date: self.to_date,
            opening_balance: self.opening_balance,
            closing_balance: self.closing_balance,
            transaction_count: self
                .transaction_count
                .ok_or_else(|| BuildError::missing_field("transaction_count"))?,
        })
    }
}
