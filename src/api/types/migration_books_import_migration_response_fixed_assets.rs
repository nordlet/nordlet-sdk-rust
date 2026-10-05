pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponseFixedAssets {
    #[serde(default)]
    pub created: i64,
    #[serde(rename = "costTotal")]
    #[serde(default)]
    pub cost_total: String,
    #[serde(rename = "accumulatedDepreciationTotal")]
    #[serde(default)]
    pub accumulated_depreciation_total: String,
}

impl BooksImportMigrationResponseFixedAssets {
    pub fn builder() -> BooksImportMigrationResponseFixedAssetsBuilder {
        <BooksImportMigrationResponseFixedAssetsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseFixedAssetsBuilder {
    created: Option<i64>,
    cost_total: Option<String>,
    accumulated_depreciation_total: Option<String>,
}

impl BooksImportMigrationResponseFixedAssetsBuilder {
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

    /// Consumes the builder and constructs a [`BooksImportMigrationResponseFixedAssets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksImportMigrationResponseFixedAssetsBuilder::created)
    /// - [`cost_total`](BooksImportMigrationResponseFixedAssetsBuilder::cost_total)
    /// - [`accumulated_depreciation_total`](BooksImportMigrationResponseFixedAssetsBuilder::accumulated_depreciation_total)
    pub fn build(self) -> Result<BooksImportMigrationResponseFixedAssets, BuildError> {
        Ok(BooksImportMigrationResponseFixedAssets {
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
