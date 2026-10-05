pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyReportConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
}

impl IntercompanyReportConsolidationRequest {
    pub fn builder() -> IntercompanyReportConsolidationRequestBuilder {
        <IntercompanyReportConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyReportConsolidationRequestBuilder {
    group_id: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
}

impl IntercompanyReportConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`IntercompanyReportConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](IntercompanyReportConsolidationRequestBuilder::group_id)
    /// - [`from_date`](IntercompanyReportConsolidationRequestBuilder::from_date)
    /// - [`to_date`](IntercompanyReportConsolidationRequestBuilder::to_date)
    pub fn build(self) -> Result<IntercompanyReportConsolidationRequest, BuildError> {
        Ok(IntercompanyReportConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
