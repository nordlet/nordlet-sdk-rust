pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SizeCategoryReportsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub criteria: SizeCategoryReportsResponseCriteria,
    pub category: SizeCategoryReportsResponseCategory,
    #[serde(default)]
    pub thresholds: HashMap<String, SizeCategoryReportsResponseThresholdsValue>,
}

impl SizeCategoryReportsResponse {
    pub fn builder() -> SizeCategoryReportsResponseBuilder {
        <SizeCategoryReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SizeCategoryReportsResponseBuilder {
    year: Option<i64>,
    criteria: Option<SizeCategoryReportsResponseCriteria>,
    category: Option<SizeCategoryReportsResponseCategory>,
    thresholds: Option<HashMap<String, SizeCategoryReportsResponseThresholdsValue>>,
}

impl SizeCategoryReportsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn criteria(mut self, value: SizeCategoryReportsResponseCriteria) -> Self {
        self.criteria = Some(value);
        self
    }

    pub fn category(mut self, value: SizeCategoryReportsResponseCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn thresholds(
        mut self,
        value: HashMap<String, SizeCategoryReportsResponseThresholdsValue>,
    ) -> Self {
        self.thresholds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SizeCategoryReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](SizeCategoryReportsResponseBuilder::year)
    /// - [`criteria`](SizeCategoryReportsResponseBuilder::criteria)
    /// - [`category`](SizeCategoryReportsResponseBuilder::category)
    /// - [`thresholds`](SizeCategoryReportsResponseBuilder::thresholds)
    pub fn build(self) -> Result<SizeCategoryReportsResponse, BuildError> {
        Ok(SizeCategoryReportsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            criteria: self
                .criteria
                .ok_or_else(|| BuildError::missing_field("criteria"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            thresholds: self
                .thresholds
                .ok_or_else(|| BuildError::missing_field("thresholds"))?,
        })
    }
}
