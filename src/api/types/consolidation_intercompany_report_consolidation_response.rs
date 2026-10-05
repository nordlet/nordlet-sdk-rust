pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyReportConsolidationResponse {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub directions: Vec<IntercompanyReportConsolidationResponseDirectionsItem>,
}

impl IntercompanyReportConsolidationResponse {
    pub fn builder() -> IntercompanyReportConsolidationResponseBuilder {
        <IntercompanyReportConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyReportConsolidationResponseBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    directions: Option<Vec<IntercompanyReportConsolidationResponseDirectionsItem>>,
}

impl IntercompanyReportConsolidationResponseBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn directions(
        mut self,
        value: Vec<IntercompanyReportConsolidationResponseDirectionsItem>,
    ) -> Self {
        self.directions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyReportConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](IntercompanyReportConsolidationResponseBuilder::from_date)
    /// - [`to_date`](IntercompanyReportConsolidationResponseBuilder::to_date)
    /// - [`directions`](IntercompanyReportConsolidationResponseBuilder::directions)
    pub fn build(self) -> Result<IntercompanyReportConsolidationResponse, BuildError> {
        Ok(IntercompanyReportConsolidationResponse {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
        })
    }
}
