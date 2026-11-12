pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReconciliationGetBankRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "statementBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_balance: Option<String>,
}

impl ReconciliationGetBankRequest {
    pub fn builder() -> ReconciliationGetBankRequestBuilder {
        <ReconciliationGetBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReconciliationGetBankRequestBuilder {
    bank_account_id: Option<String>,
    date: Option<NaiveDate>,
    statement_balance: Option<String>,
}

impl ReconciliationGetBankRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ReconciliationGetBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](ReconciliationGetBankRequestBuilder::bank_account_id)
    /// - [`date`](ReconciliationGetBankRequestBuilder::date)
    pub fn build(self) -> Result<ReconciliationGetBankRequest, BuildError> {
        Ok(ReconciliationGetBankRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            statement_balance: self.statement_balance,
        })
    }
}
