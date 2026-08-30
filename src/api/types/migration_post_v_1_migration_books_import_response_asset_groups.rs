pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponseAssetGroups {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl PostV1MigrationBooksImportResponseAssetGroups {
    pub fn builder() -> PostV1MigrationBooksImportResponseAssetGroupsBuilder {
        <PostV1MigrationBooksImportResponseAssetGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseAssetGroupsBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl PostV1MigrationBooksImportResponseAssetGroupsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponseAssetGroups`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksImportResponseAssetGroupsBuilder::created)
    /// - [`existing`](PostV1MigrationBooksImportResponseAssetGroupsBuilder::existing)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponseAssetGroups, BuildError> {
        Ok(PostV1MigrationBooksImportResponseAssetGroups {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
