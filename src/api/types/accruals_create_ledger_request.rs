pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccrualsCreateLedgerRequest {
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub lines: Vec<AccrualsCreateLedgerRequestLinesItem>,
}

impl AccrualsCreateLedgerRequest {
    pub fn builder() -> AccrualsCreateLedgerRequestBuilder {
        <AccrualsCreateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccrualsCreateLedgerRequestBuilder {
    date: Option<NaiveDate>,
    reason: Option<String>,
    lines: Option<Vec<AccrualsCreateLedgerRequestLinesItem>>,
}

impl AccrualsCreateLedgerRequestBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<AccrualsCreateLedgerRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccrualsCreateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](AccrualsCreateLedgerRequestBuilder::date)
    /// - [`reason`](AccrualsCreateLedgerRequestBuilder::reason)
    /// - [`lines`](AccrualsCreateLedgerRequestBuilder::lines)
    pub fn build(self) -> Result<AccrualsCreateLedgerRequest, BuildError> {
        Ok(AccrualsCreateLedgerRequest {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
