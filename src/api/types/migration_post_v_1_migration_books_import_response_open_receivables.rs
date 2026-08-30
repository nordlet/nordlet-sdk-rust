pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponseOpenReceivables {
    #[serde(default)]
    pub created: i64,
    #[serde(rename = "outstandingTotal")]
    #[serde(default)]
    pub outstanding_total: String,
}

impl PostV1MigrationBooksImportResponseOpenReceivables {
    pub fn builder() -> PostV1MigrationBooksImportResponseOpenReceivablesBuilder {
        <PostV1MigrationBooksImportResponseOpenReceivablesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseOpenReceivablesBuilder {
    created: Option<i64>,
    outstanding_total: Option<String>,
}

impl PostV1MigrationBooksImportResponseOpenReceivablesBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn outstanding_total(mut self, value: impl Into<String>) -> Self {
        self.outstanding_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponseOpenReceivables`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksImportResponseOpenReceivablesBuilder::created)
    /// - [`outstanding_total`](PostV1MigrationBooksImportResponseOpenReceivablesBuilder::outstanding_total)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponseOpenReceivables, BuildError> {
        Ok(PostV1MigrationBooksImportResponseOpenReceivables {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            outstanding_total: self
                .outstanding_total
                .ok_or_else(|| BuildError::missing_field("outstanding_total"))?,
        })
    }
}
