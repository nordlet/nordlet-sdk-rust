pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepreciationPreviewAssetsResponse {
    #[serde(default)]
    pub rows: Vec<DepreciationPreviewAssetsResponseRowsItem>,
    #[serde(default)]
    pub total: String,
}

impl DepreciationPreviewAssetsResponse {
    pub fn builder() -> DepreciationPreviewAssetsResponseBuilder {
        <DepreciationPreviewAssetsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepreciationPreviewAssetsResponseBuilder {
    rows: Option<Vec<DepreciationPreviewAssetsResponseRowsItem>>,
    total: Option<String>,
}

impl DepreciationPreviewAssetsResponseBuilder {
    pub fn rows(mut self, value: Vec<DepreciationPreviewAssetsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DepreciationPreviewAssetsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](DepreciationPreviewAssetsResponseBuilder::rows)
    /// - [`total`](DepreciationPreviewAssetsResponseBuilder::total)
    pub fn build(self) -> Result<DepreciationPreviewAssetsResponse, BuildError> {
        Ok(DepreciationPreviewAssetsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
