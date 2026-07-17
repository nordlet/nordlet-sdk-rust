pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsSizeCategoryRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1ReportsSizeCategoryRequest {
    pub fn builder() -> PostV1ReportsSizeCategoryRequestBuilder {
        <PostV1ReportsSizeCategoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsSizeCategoryRequestBuilder {
    year: Option<i64>,
}

impl PostV1ReportsSizeCategoryRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsSizeCategoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1ReportsSizeCategoryRequestBuilder::year)
    pub fn build(self) -> Result<PostV1ReportsSizeCategoryRequest, BuildError> {
        Ok(PostV1ReportsSizeCategoryRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
