pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponseItems {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl PostV1MigrationBooksImportResponseItems {
    pub fn builder() -> PostV1MigrationBooksImportResponseItemsBuilder {
        <PostV1MigrationBooksImportResponseItemsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseItemsBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl PostV1MigrationBooksImportResponseItemsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponseItems`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksImportResponseItemsBuilder::created)
    /// - [`existing`](PostV1MigrationBooksImportResponseItemsBuilder::existing)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponseItems, BuildError> {
        Ok(PostV1MigrationBooksImportResponseItems {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
