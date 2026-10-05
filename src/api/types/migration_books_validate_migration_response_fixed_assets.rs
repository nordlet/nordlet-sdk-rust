pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationResponseFixedAssets {
    #[serde(default)]
    pub created: i64,
    #[serde(rename = "costTotal")]
    #[serde(default)]
    pub cost_total: String,
    #[serde(rename = "accumulatedDepreciationTotal")]
    #[serde(default)]
    pub accumulated_depreciation_total: String,
}

impl BooksValidateMigrationResponseFixedAssets {
    pub fn builder() -> BooksValidateMigrationResponseFixedAssetsBuilder {
        <BooksValidateMigrationResponseFixedAssetsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationResponseFixedAssetsBuilder {
    created: Option<i64>,
    cost_total: Option<String>,
    accumulated_depreciation_total: Option<String>,
}

impl BooksValidateMigrationResponseFixedAssetsBuilder {
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

    /// Consumes the builder and constructs a [`BooksValidateMigrationResponseFixedAssets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksValidateMigrationResponseFixedAssetsBuilder::created)
    /// - [`cost_total`](BooksValidateMigrationResponseFixedAssetsBuilder::cost_total)
    /// - [`accumulated_depreciation_total`](BooksValidateMigrationResponseFixedAssetsBuilder::accumulated_depreciation_total)
    pub fn build(self) -> Result<BooksValidateMigrationResponseFixedAssets, BuildError> {
        Ok(BooksValidateMigrationResponseFixedAssets {
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
