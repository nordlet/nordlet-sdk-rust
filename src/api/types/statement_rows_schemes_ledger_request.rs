pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementRowsSchemesLedgerRequest {}

impl StatementRowsSchemesLedgerRequest {
    pub fn builder() -> StatementRowsSchemesLedgerRequestBuilder {
        <StatementRowsSchemesLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsSchemesLedgerRequestBuilder {}

impl StatementRowsSchemesLedgerRequestBuilder {
    /// Consumes the builder and constructs a [`StatementRowsSchemesLedgerRequest`].
    pub fn build(self) -> Result<StatementRowsSchemesLedgerRequest, BuildError> {
        Ok(StatementRowsSchemesLedgerRequest {})
    }
}
