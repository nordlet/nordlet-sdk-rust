pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponseItems {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl BooksImportMigrationResponseItems {
    pub fn builder() -> BooksImportMigrationResponseItemsBuilder {
        <BooksImportMigrationResponseItemsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseItemsBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl BooksImportMigrationResponseItemsBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationResponseItems`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksImportMigrationResponseItemsBuilder::created)
    /// - [`existing`](BooksImportMigrationResponseItemsBuilder::existing)
    pub fn build(self) -> Result<BooksImportMigrationResponseItems, BuildError> {
        Ok(BooksImportMigrationResponseItems {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
