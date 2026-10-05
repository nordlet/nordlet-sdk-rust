pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TrialBalanceReportsResponseTotals {
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
}

impl TrialBalanceReportsResponseTotals {
    pub fn builder() -> TrialBalanceReportsResponseTotalsBuilder {
        <TrialBalanceReportsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrialBalanceReportsResponseTotalsBuilder {
    debit: Option<String>,
    credit: Option<String>,
}

impl TrialBalanceReportsResponseTotalsBuilder {
    pub fn debit(mut self, value: impl Into<String>) -> Self {
        self.debit = Some(value.into());
        self
    }

    pub fn credit(mut self, value: impl Into<String>) -> Self {
        self.credit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TrialBalanceReportsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`debit`](TrialBalanceReportsResponseTotalsBuilder::debit)
    /// - [`credit`](TrialBalanceReportsResponseTotalsBuilder::credit)
    pub fn build(self) -> Result<TrialBalanceReportsResponseTotals, BuildError> {
        Ok(TrialBalanceReportsResponseTotals {
            debit: self
                .debit
                .ok_or_else(|| BuildError::missing_field("debit"))?,
            credit: self
                .credit
                .ok_or_else(|| BuildError::missing_field("credit"))?,
        })
    }
}
