pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationResponseAccounts {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl BooksValidateMigrationResponseAccounts {
    pub fn builder() -> BooksValidateMigrationResponseAccountsBuilder {
        <BooksValidateMigrationResponseAccountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationResponseAccountsBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl BooksValidateMigrationResponseAccountsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationResponseAccounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksValidateMigrationResponseAccountsBuilder::created)
    /// - [`existing`](BooksValidateMigrationResponseAccountsBuilder::existing)
    pub fn build(self) -> Result<BooksValidateMigrationResponseAccounts, BuildError> {
        Ok(BooksValidateMigrationResponseAccounts {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
