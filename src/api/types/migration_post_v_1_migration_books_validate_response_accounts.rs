pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateResponseAccounts {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl PostV1MigrationBooksValidateResponseAccounts {
    pub fn builder() -> PostV1MigrationBooksValidateResponseAccountsBuilder {
        <PostV1MigrationBooksValidateResponseAccountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateResponseAccountsBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl PostV1MigrationBooksValidateResponseAccountsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateResponseAccounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksValidateResponseAccountsBuilder::created)
    /// - [`existing`](PostV1MigrationBooksValidateResponseAccountsBuilder::existing)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateResponseAccounts, BuildError> {
        Ok(PostV1MigrationBooksValidateResponseAccounts {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
