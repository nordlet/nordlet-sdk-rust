pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponseAccounts {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl BooksImportMigrationResponseAccounts {
    pub fn builder() -> BooksImportMigrationResponseAccountsBuilder {
        <BooksImportMigrationResponseAccountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseAccountsBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl BooksImportMigrationResponseAccountsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationResponseAccounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksImportMigrationResponseAccountsBuilder::created)
    /// - [`existing`](BooksImportMigrationResponseAccountsBuilder::existing)
    pub fn build(self) -> Result<BooksImportMigrationResponseAccounts, BuildError> {
        Ok(BooksImportMigrationResponseAccounts {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
