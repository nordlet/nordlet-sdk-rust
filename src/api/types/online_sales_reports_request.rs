pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OnlineSalesReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
}

impl OnlineSalesReportsRequest {
    pub fn builder() -> OnlineSalesReportsRequestBuilder {
        <OnlineSalesReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OnlineSalesReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
}

impl OnlineSalesReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OnlineSalesReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](OnlineSalesReportsRequestBuilder::from_date)
    /// - [`to_date`](OnlineSalesReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<OnlineSalesReportsRequest, BuildError> {
        Ok(OnlineSalesReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
