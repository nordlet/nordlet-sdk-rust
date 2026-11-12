pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionsMatchBankRequestRemainder {
    pub treatment: TransactionsMatchBankRequestRemainderTreatment,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
}

impl TransactionsMatchBankRequestRemainder {
    pub fn builder() -> TransactionsMatchBankRequestRemainderBuilder {
        <TransactionsMatchBankRequestRemainderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsMatchBankRequestRemainderBuilder {
    treatment: Option<TransactionsMatchBankRequestRemainderTreatment>,
    account_code: Option<String>,
}

impl TransactionsMatchBankRequestRemainderBuilder {
    pub fn treatment(mut self, value: TransactionsMatchBankRequestRemainderTreatment) -> Self {
        self.treatment = Some(value);
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionsMatchBankRequestRemainder`].
    /// This method will fail if any of the following fields are not set:
    /// - [`treatment`](TransactionsMatchBankRequestRemainderBuilder::treatment)
    pub fn build(self) -> Result<TransactionsMatchBankRequestRemainder, BuildError> {
        Ok(TransactionsMatchBankRequestRemainder {
            treatment: self
                .treatment
                .ok_or_else(|| BuildError::missing_field("treatment"))?,
            account_code: self.account_code,
        })
    }
}
