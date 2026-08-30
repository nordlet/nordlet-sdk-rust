pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponseOpenPayables {
    #[serde(default)]
    pub created: i64,
    #[serde(rename = "outstandingTotal")]
    #[serde(default)]
    pub outstanding_total: String,
}

impl PostV1MigrationBooksImportResponseOpenPayables {
    pub fn builder() -> PostV1MigrationBooksImportResponseOpenPayablesBuilder {
        <PostV1MigrationBooksImportResponseOpenPayablesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseOpenPayablesBuilder {
    created: Option<i64>,
    outstanding_total: Option<String>,
}

impl PostV1MigrationBooksImportResponseOpenPayablesBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn outstanding_total(mut self, value: impl Into<String>) -> Self {
        self.outstanding_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponseOpenPayables`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksImportResponseOpenPayablesBuilder::created)
    /// - [`outstanding_total`](PostV1MigrationBooksImportResponseOpenPayablesBuilder::outstanding_total)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponseOpenPayables, BuildError> {
        Ok(PostV1MigrationBooksImportResponseOpenPayables {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            outstanding_total: self
                .outstanding_total
                .ok_or_else(|| BuildError::missing_field("outstanding_total"))?,
        })
    }
}
