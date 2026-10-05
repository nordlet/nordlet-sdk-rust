pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SieReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "includeTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transactions: Option<bool>,
}

impl SieReportsRequest {
    pub fn builder() -> SieReportsRequestBuilder {
        <SieReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SieReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    include_transactions: Option<bool>,
}

impl SieReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn include_transactions(mut self, value: bool) -> Self {
        self.include_transactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SieReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](SieReportsRequestBuilder::from_date)
    /// - [`to_date`](SieReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<SieReportsRequest, BuildError> {
        Ok(SieReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            include_transactions: self.include_transactions,
        })
    }
}
