pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponseAccounts {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl PostV1MigrationBooksImportResponseAccounts {
    pub fn builder() -> PostV1MigrationBooksImportResponseAccountsBuilder {
        <PostV1MigrationBooksImportResponseAccountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseAccountsBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl PostV1MigrationBooksImportResponseAccountsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponseAccounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksImportResponseAccountsBuilder::created)
    /// - [`existing`](PostV1MigrationBooksImportResponseAccountsBuilder::existing)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponseAccounts, BuildError> {
        Ok(PostV1MigrationBooksImportResponseAccounts {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
