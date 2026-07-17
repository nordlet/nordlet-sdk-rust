pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsDepreciationPreviewResponse {
    #[serde(default)]
    pub rows: Vec<PostV1AssetsDepreciationPreviewResponseRowsItem>,
    #[serde(default)]
    pub total: String,
}

impl PostV1AssetsDepreciationPreviewResponse {
    pub fn builder() -> PostV1AssetsDepreciationPreviewResponseBuilder {
        <PostV1AssetsDepreciationPreviewResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsDepreciationPreviewResponseBuilder {
    rows: Option<Vec<PostV1AssetsDepreciationPreviewResponseRowsItem>>,
    total: Option<String>,
}

impl PostV1AssetsDepreciationPreviewResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1AssetsDepreciationPreviewResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsDepreciationPreviewResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1AssetsDepreciationPreviewResponseBuilder::rows)
    /// - [`total`](PostV1AssetsDepreciationPreviewResponseBuilder::total)
    pub fn build(self) -> Result<PostV1AssetsDepreciationPreviewResponse, BuildError> {
        Ok(PostV1AssetsDepreciationPreviewResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
