pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationResponseAssetGroups {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl BooksValidateMigrationResponseAssetGroups {
    pub fn builder() -> BooksValidateMigrationResponseAssetGroupsBuilder {
        <BooksValidateMigrationResponseAssetGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationResponseAssetGroupsBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl BooksValidateMigrationResponseAssetGroupsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationResponseAssetGroups`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksValidateMigrationResponseAssetGroupsBuilder::created)
    /// - [`existing`](BooksValidateMigrationResponseAssetGroupsBuilder::existing)
    pub fn build(self) -> Result<BooksValidateMigrationResponseAssetGroups, BuildError> {
        Ok(BooksValidateMigrationResponseAssetGroups {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
