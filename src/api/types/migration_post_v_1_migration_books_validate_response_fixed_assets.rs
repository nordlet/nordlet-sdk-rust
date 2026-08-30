pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateResponseFixedAssets {
    #[serde(default)]
    pub created: i64,
    #[serde(rename = "costTotal")]
    #[serde(default)]
    pub cost_total: String,
    #[serde(rename = "accumulatedDepreciationTotal")]
    #[serde(default)]
    pub accumulated_depreciation_total: String,
}

impl PostV1MigrationBooksValidateResponseFixedAssets {
    pub fn builder() -> PostV1MigrationBooksValidateResponseFixedAssetsBuilder {
        <PostV1MigrationBooksValidateResponseFixedAssetsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateResponseFixedAssetsBuilder {
    created: Option<i64>,
    cost_total: Option<String>,
    accumulated_depreciation_total: Option<String>,
}

impl PostV1MigrationBooksValidateResponseFixedAssetsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn cost_total(mut self, value: impl Into<String>) -> Self {
        self.cost_total = Some(value.into());
        self
    }

    pub fn accumulated_depreciation_total(mut self, value: impl Into<String>) -> Self {
        self.accumulated_depreciation_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateResponseFixedAssets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksValidateResponseFixedAssetsBuilder::created)
    /// - [`cost_total`](PostV1MigrationBooksValidateResponseFixedAssetsBuilder::cost_total)
    /// - [`accumulated_depreciation_total`](PostV1MigrationBooksValidateResponseFixedAssetsBuilder::accumulated_depreciation_total)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateResponseFixedAssets, BuildError> {
        Ok(PostV1MigrationBooksValidateResponseFixedAssets {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            cost_total: self
                .cost_total
                .ok_or_else(|| BuildError::missing_field("cost_total"))?,
            accumulated_depreciation_total: self
                .accumulated_depreciation_total
                .ok_or_else(|| BuildError::missing_field("accumulated_depreciation_total"))?,
        })
    }
}
