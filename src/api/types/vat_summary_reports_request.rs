pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSummaryReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<VatSummaryReportsRequestSide>,
}

impl VatSummaryReportsRequest {
    pub fn builder() -> VatSummaryReportsRequestBuilder {
        <VatSummaryReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSummaryReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    side: Option<VatSummaryReportsRequestSide>,
}

impl VatSummaryReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn side(mut self, value: VatSummaryReportsRequestSide) -> Self {
        self.side = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatSummaryReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](VatSummaryReportsRequestBuilder::from_date)
    /// - [`to_date`](VatSummaryReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<VatSummaryReportsRequest, BuildError> {
        Ok(VatSummaryReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            side: self.side,
        })
    }
}
