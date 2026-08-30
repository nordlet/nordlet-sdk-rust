pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateResponseOpenPayables {
    #[serde(default)]
    pub created: i64,
    #[serde(rename = "outstandingTotal")]
    #[serde(default)]
    pub outstanding_total: String,
}

impl PostV1MigrationBooksValidateResponseOpenPayables {
    pub fn builder() -> PostV1MigrationBooksValidateResponseOpenPayablesBuilder {
        <PostV1MigrationBooksValidateResponseOpenPayablesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateResponseOpenPayablesBuilder {
    created: Option<i64>,
    outstanding_total: Option<String>,
}

impl PostV1MigrationBooksValidateResponseOpenPayablesBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn outstanding_total(mut self, value: impl Into<String>) -> Self {
        self.outstanding_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateResponseOpenPayables`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksValidateResponseOpenPayablesBuilder::created)
    /// - [`outstanding_total`](PostV1MigrationBooksValidateResponseOpenPayablesBuilder::outstanding_total)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateResponseOpenPayables, BuildError> {
        Ok(PostV1MigrationBooksValidateResponseOpenPayables {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            outstanding_total: self
                .outstanding_total
                .ok_or_else(|| BuildError::missing_field("outstanding_total"))?,
        })
    }
}
