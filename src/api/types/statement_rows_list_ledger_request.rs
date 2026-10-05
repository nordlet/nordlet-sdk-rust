pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementRowsListLedgerRequest {
    #[serde(default)]
    pub scheme: String,
    #[serde(rename = "fromDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<NaiveDate>,
    #[serde(rename = "toDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<NaiveDate>,
}

impl StatementRowsListLedgerRequest {
    pub fn builder() -> StatementRowsListLedgerRequestBuilder {
        <StatementRowsListLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsListLedgerRequestBuilder {
    scheme: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
}

impl StatementRowsListLedgerRequestBuilder {
    pub fn scheme(mut self, value: impl Into<String>) -> Self {
        self.scheme = Some(value.into());
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

    /// Consumes the builder and constructs a [`StatementRowsListLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheme`](StatementRowsListLedgerRequestBuilder::scheme)
    pub fn build(self) -> Result<StatementRowsListLedgerRequest, BuildError> {
        Ok(StatementRowsListLedgerRequest {
            scheme: self
                .scheme
                .ok_or_else(|| BuildError::missing_field("scheme"))?,
            from_date: self.from_date,
            to_date: self.to_date,
        })
    }
}
