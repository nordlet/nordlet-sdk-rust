pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PosSalesReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
}

impl PosSalesReportsRequest {
    pub fn builder() -> PosSalesReportsRequestBuilder {
        <PosSalesReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PosSalesReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
}

impl PosSalesReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PosSalesReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PosSalesReportsRequestBuilder::from_date)
    /// - [`to_date`](PosSalesReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<PosSalesReportsRequest, BuildError> {
        Ok(PosSalesReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
