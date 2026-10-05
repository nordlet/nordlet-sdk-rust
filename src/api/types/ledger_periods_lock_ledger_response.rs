pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeriodsLockLedgerResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub status: PeriodsLockLedgerResponseStatus,
}

impl PeriodsLockLedgerResponse {
    pub fn builder() -> PeriodsLockLedgerResponseBuilder {
        <PeriodsLockLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodsLockLedgerResponseBuilder {
    id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    status: Option<PeriodsLockLedgerResponseStatus>,
}

impl PeriodsLockLedgerResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn status(mut self, value: PeriodsLockLedgerResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeriodsLockLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PeriodsLockLedgerResponseBuilder::id)
    /// - [`year`](PeriodsLockLedgerResponseBuilder::year)
    /// - [`month`](PeriodsLockLedgerResponseBuilder::month)
    /// - [`status`](PeriodsLockLedgerResponseBuilder::status)
    pub fn build(self) -> Result<PeriodsLockLedgerResponse, BuildError> {
        Ok(PeriodsLockLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
