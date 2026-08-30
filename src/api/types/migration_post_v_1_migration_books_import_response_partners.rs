pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponsePartners {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl PostV1MigrationBooksImportResponsePartners {
    pub fn builder() -> PostV1MigrationBooksImportResponsePartnersBuilder {
        <PostV1MigrationBooksImportResponsePartnersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponsePartnersBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl PostV1MigrationBooksImportResponsePartnersBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponsePartners`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksImportResponsePartnersBuilder::created)
    /// - [`existing`](PostV1MigrationBooksImportResponsePartnersBuilder::existing)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponsePartners, BuildError> {
        Ok(PostV1MigrationBooksImportResponsePartners {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
