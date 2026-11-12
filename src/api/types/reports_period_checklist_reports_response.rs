pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PeriodChecklistReportsResponse {
    #[serde(default)]
    pub period: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub items: Vec<PeriodChecklistReportsResponseItemsItem>,
}

impl PeriodChecklistReportsResponse {
    pub fn builder() -> PeriodChecklistReportsResponseBuilder {
        <PeriodChecklistReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodChecklistReportsResponseBuilder {
    period: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    items: Option<Vec<PeriodChecklistReportsResponseItemsItem>>,
}

impl PeriodChecklistReportsResponseBuilder {
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
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

    pub fn items(mut self, value: Vec<PeriodChecklistReportsResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeriodChecklistReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period`](PeriodChecklistReportsResponseBuilder::period)
    /// - [`from_date`](PeriodChecklistReportsResponseBuilder::from_date)
    /// - [`to_date`](PeriodChecklistReportsResponseBuilder::to_date)
    /// - [`items`](PeriodChecklistReportsResponseBuilder::items)
    pub fn build(self) -> Result<PeriodChecklistReportsResponse, BuildError> {
        Ok(PeriodChecklistReportsResponse {
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
