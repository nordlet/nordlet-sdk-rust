pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsTrialBalanceResponseTotals {
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
}

impl PostV1ReportsTrialBalanceResponseTotals {
    pub fn builder() -> PostV1ReportsTrialBalanceResponseTotalsBuilder {
        <PostV1ReportsTrialBalanceResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsTrialBalanceResponseTotalsBuilder {
    debit: Option<String>,
    credit: Option<String>,
}

impl PostV1ReportsTrialBalanceResponseTotalsBuilder {
    pub fn debit(mut self, value: impl Into<String>) -> Self {
        self.debit = Some(value.into());
        self
    }

    pub fn credit(mut self, value: impl Into<String>) -> Self {
        self.credit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsTrialBalanceResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`debit`](PostV1ReportsTrialBalanceResponseTotalsBuilder::debit)
    /// - [`credit`](PostV1ReportsTrialBalanceResponseTotalsBuilder::credit)
    pub fn build(self) -> Result<PostV1ReportsTrialBalanceResponseTotals, BuildError> {
        Ok(PostV1ReportsTrialBalanceResponseTotals {
            debit: self
                .debit
                .ok_or_else(|| BuildError::missing_field("debit"))?,
            credit: self
                .credit
                .ok_or_else(|| BuildError::missing_field("credit"))?,
        })
    }
}
