pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponseOpenPayables {
    #[serde(default)]
    pub created: i64,
    #[serde(rename = "outstandingTotal")]
    #[serde(default)]
    pub outstanding_total: String,
}

impl BooksImportMigrationResponseOpenPayables {
    pub fn builder() -> BooksImportMigrationResponseOpenPayablesBuilder {
        <BooksImportMigrationResponseOpenPayablesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseOpenPayablesBuilder {
    created: Option<i64>,
    outstanding_total: Option<String>,
}

impl BooksImportMigrationResponseOpenPayablesBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn outstanding_total(mut self, value: impl Into<String>) -> Self {
        self.outstanding_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationResponseOpenPayables`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksImportMigrationResponseOpenPayablesBuilder::created)
    /// - [`outstanding_total`](BooksImportMigrationResponseOpenPayablesBuilder::outstanding_total)
    pub fn build(self) -> Result<BooksImportMigrationResponseOpenPayables, BuildError> {
        Ok(BooksImportMigrationResponseOpenPayables {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            outstanding_total: self
                .outstanding_total
                .ok_or_else(|| BuildError::missing_field("outstanding_total"))?,
        })
    }
}
