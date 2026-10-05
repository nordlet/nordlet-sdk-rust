pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationResponsePartners {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub existing: i64,
}

impl BooksValidateMigrationResponsePartners {
    pub fn builder() -> BooksValidateMigrationResponsePartnersBuilder {
        <BooksValidateMigrationResponsePartnersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationResponsePartnersBuilder {
    created: Option<i64>,
    existing: Option<i64>,
}

impl BooksValidateMigrationResponsePartnersBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationResponsePartners`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](BooksValidateMigrationResponsePartnersBuilder::created)
    /// - [`existing`](BooksValidateMigrationResponsePartnersBuilder::existing)
    pub fn build(self) -> Result<BooksValidateMigrationResponsePartners, BuildError> {
        Ok(BooksValidateMigrationResponsePartners {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            existing: self
                .existing
                .ok_or_else(|| BuildError::missing_field("existing"))?,
        })
    }
}
