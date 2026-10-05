pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TrialBalanceReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
}

impl TrialBalanceReportsRequest {
    pub fn builder() -> TrialBalanceReportsRequestBuilder {
        <TrialBalanceReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrialBalanceReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
}

impl TrialBalanceReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TrialBalanceReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](TrialBalanceReportsRequestBuilder::from_date)
    /// - [`to_date`](TrialBalanceReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<TrialBalanceReportsRequest, BuildError> {
        Ok(TrialBalanceReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
