pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PeriodsUnlockLedgerRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PeriodsUnlockLedgerRequest {
    pub fn builder() -> PeriodsUnlockLedgerRequestBuilder {
        <PeriodsUnlockLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodsUnlockLedgerRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PeriodsUnlockLedgerRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeriodsUnlockLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PeriodsUnlockLedgerRequestBuilder::year)
    /// - [`month`](PeriodsUnlockLedgerRequestBuilder::month)
    pub fn build(self) -> Result<PeriodsUnlockLedgerRequest, BuildError> {
        Ok(PeriodsUnlockLedgerRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
