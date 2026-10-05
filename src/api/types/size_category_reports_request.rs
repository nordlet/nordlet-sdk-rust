pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SizeCategoryReportsRequest {
    #[serde(default)]
    pub year: i64,
}

impl SizeCategoryReportsRequest {
    pub fn builder() -> SizeCategoryReportsRequestBuilder {
        <SizeCategoryReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SizeCategoryReportsRequestBuilder {
    year: Option<i64>,
}

impl SizeCategoryReportsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SizeCategoryReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](SizeCategoryReportsRequestBuilder::year)
    pub fn build(self) -> Result<SizeCategoryReportsRequest, BuildError> {
        Ok(SizeCategoryReportsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
