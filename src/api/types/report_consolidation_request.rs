pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ReportConsolidationRequestCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eliminations: Option<Vec<ReportConsolidationRequestEliminationsItem>>,
}

impl ReportConsolidationRequest {
    pub fn builder() -> ReportConsolidationRequestBuilder {
        <ReportConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationRequestBuilder {
    group_id: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    category: Option<ReportConsolidationRequestCategory>,
    eliminations: Option<Vec<ReportConsolidationRequestEliminationsItem>>,
}

impl ReportConsolidationRequestBuilder {
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

    pub fn category(mut self, value: ReportConsolidationRequestCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn eliminations(mut self, value: Vec<ReportConsolidationRequestEliminationsItem>) -> Self {
        self.eliminations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](ReportConsolidationRequestBuilder::group_id)
    /// - [`from_date`](ReportConsolidationRequestBuilder::from_date)
    /// - [`to_date`](ReportConsolidationRequestBuilder::to_date)
    pub fn build(self) -> Result<ReportConsolidationRequest, BuildError> {
        Ok(ReportConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            category: self.category,
            eliminations: self.eliminations,
        })
    }
}
