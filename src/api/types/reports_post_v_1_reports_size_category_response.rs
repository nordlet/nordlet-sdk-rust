pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReportsSizeCategoryResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub criteria: PostV1ReportsSizeCategoryResponseCriteria,
    pub category: PostV1ReportsSizeCategoryResponseCategory,
    #[serde(default)]
    pub thresholds: HashMap<String, PostV1ReportsSizeCategoryResponseThresholdsValue>,
}

impl PostV1ReportsSizeCategoryResponse {
    pub fn builder() -> PostV1ReportsSizeCategoryResponseBuilder {
        <PostV1ReportsSizeCategoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsSizeCategoryResponseBuilder {
    year: Option<i64>,
    criteria: Option<PostV1ReportsSizeCategoryResponseCriteria>,
    category: Option<PostV1ReportsSizeCategoryResponseCategory>,
    thresholds: Option<HashMap<String, PostV1ReportsSizeCategoryResponseThresholdsValue>>,
}

impl PostV1ReportsSizeCategoryResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn criteria(mut self, value: PostV1ReportsSizeCategoryResponseCriteria) -> Self {
        self.criteria = Some(value);
        self
    }

    pub fn category(mut self, value: PostV1ReportsSizeCategoryResponseCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn thresholds(
        mut self,
        value: HashMap<String, PostV1ReportsSizeCategoryResponseThresholdsValue>,
    ) -> Self {
        self.thresholds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsSizeCategoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1ReportsSizeCategoryResponseBuilder::year)
    /// - [`criteria`](PostV1ReportsSizeCategoryResponseBuilder::criteria)
    /// - [`category`](PostV1ReportsSizeCategoryResponseBuilder::category)
    /// - [`thresholds`](PostV1ReportsSizeCategoryResponseBuilder::thresholds)
    pub fn build(self) -> Result<PostV1ReportsSizeCategoryResponse, BuildError> {
        Ok(PostV1ReportsSizeCategoryResponse {
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
