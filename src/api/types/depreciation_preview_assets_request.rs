pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepreciationPreviewAssetsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl DepreciationPreviewAssetsRequest {
    pub fn builder() -> DepreciationPreviewAssetsRequestBuilder {
        <DepreciationPreviewAssetsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepreciationPreviewAssetsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl DepreciationPreviewAssetsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DepreciationPreviewAssetsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DepreciationPreviewAssetsRequestBuilder::year)
    /// - [`month`](DepreciationPreviewAssetsRequestBuilder::month)
    pub fn build(self) -> Result<DepreciationPreviewAssetsRequest, BuildError> {
        Ok(DepreciationPreviewAssetsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
