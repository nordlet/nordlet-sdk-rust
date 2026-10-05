pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponseNumberSeriesItem {
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "nextNumber")]
    #[serde(default)]
    pub next_number: i64,
}

impl BooksImportMigrationResponseNumberSeriesItem {
    pub fn builder() -> BooksImportMigrationResponseNumberSeriesItemBuilder {
        <BooksImportMigrationResponseNumberSeriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseNumberSeriesItemBuilder {
    prefix: Option<String>,
    year: Option<i64>,
    next_number: Option<i64>,
}

impl BooksImportMigrationResponseNumberSeriesItemBuilder {
    pub fn prefix(mut self, value: impl Into<String>) -> Self {
        self.prefix = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn next_number(mut self, value: i64) -> Self {
        self.next_number = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationResponseNumberSeriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prefix`](BooksImportMigrationResponseNumberSeriesItemBuilder::prefix)
    /// - [`year`](BooksImportMigrationResponseNumberSeriesItemBuilder::year)
    /// - [`next_number`](BooksImportMigrationResponseNumberSeriesItemBuilder::next_number)
    pub fn build(self) -> Result<BooksImportMigrationResponseNumberSeriesItem, BuildError> {
        Ok(BooksImportMigrationResponseNumberSeriesItem {
            prefix: self
                .prefix
                .ok_or_else(|| BuildError::missing_field("prefix"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            next_number: self
                .next_number
                .ok_or_else(|| BuildError::missing_field("next_number"))?,
        })
    }
}
