pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateResponsePartners {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl PostV1MigrationBooksValidateResponsePartners {
    pub fn builder() -> PostV1MigrationBooksValidateResponsePartnersBuilder {
        <PostV1MigrationBooksValidateResponsePartnersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateResponsePartnersBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl PostV1MigrationBooksValidateResponsePartnersBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateResponsePartners`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1MigrationBooksValidateResponsePartnersBuilder::created)
    /// - [`existing`](PostV1MigrationBooksValidateResponsePartnersBuilder::existing)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateResponsePartners, BuildError> {
        Ok(PostV1MigrationBooksValidateResponsePartners {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
