pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponseOpenReceivables {
    #[serde(default)]
    pub created: i64,
    #[serde(rename = "outstandingTotal")]
    #[serde(default)]
    pub outstanding_total: String,
}

impl BooksImportMigrationResponseOpenReceivables {
    pub fn builder() -> BooksImportMigrationResponseOpenReceivablesBuilder {
        <BooksImportMigrationResponseOpenReceivablesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseOpenReceivablesBuilder {
    created: Option<i64>,
    outstanding_total: Option<String>,
}

impl BooksImportMigrationResponseOpenReceivablesBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn outstanding_total(mut self, value: impl Into<String>) -> Self {
        self.outstanding_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationResponseOpenReceivables`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksImportMigrationResponseOpenReceivablesBuilder::created)
    /// - [`outstanding_total`](BooksImportMigrationResponseOpenReceivablesBuilder::outstanding_total)
    pub fn build(self) -> Result<BooksImportMigrationResponseOpenReceivables, BuildError> {
        Ok(BooksImportMigrationResponseOpenReceivables {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            outstanding_total: self
                .outstanding_total
                .ok_or_else(|| BuildError::missing_field("outstanding_total"))?,
        })
    }
}
