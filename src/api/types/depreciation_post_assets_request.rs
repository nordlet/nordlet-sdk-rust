pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepreciationPostAssetsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl DepreciationPostAssetsRequest {
    pub fn builder() -> DepreciationPostAssetsRequestBuilder {
        <DepreciationPostAssetsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepreciationPostAssetsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl DepreciationPostAssetsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DepreciationPostAssetsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DepreciationPostAssetsRequestBuilder::year)
    /// - [`month`](DepreciationPostAssetsRequestBuilder::month)
    pub fn build(self) -> Result<DepreciationPostAssetsRequest, BuildError> {
        Ok(DepreciationPostAssetsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
