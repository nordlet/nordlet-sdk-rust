pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementRowsSchemesLedgerResponse {
    #[serde(default)]
    pub rows: Vec<StatementRowsSchemesLedgerResponseRowsItem>,
}

impl StatementRowsSchemesLedgerResponse {
    pub fn builder() -> StatementRowsSchemesLedgerResponseBuilder {
        <StatementRowsSchemesLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsSchemesLedgerResponseBuilder {
    rows: Option<Vec<StatementRowsSchemesLedgerResponseRowsItem>>,
}

impl StatementRowsSchemesLedgerResponseBuilder {
    pub fn rows(mut self, value: Vec<StatementRowsSchemesLedgerResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementRowsSchemesLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](StatementRowsSchemesLedgerResponseBuilder::rows)
    pub fn build(self) -> Result<StatementRowsSchemesLedgerResponse, BuildError> {
        Ok(StatementRowsSchemesLedgerResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
