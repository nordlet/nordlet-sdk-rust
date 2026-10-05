pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponseStock {
    #[serde(default)]
    pub movements: i64,
    #[serde(rename = "costTotal")]
    #[serde(default)]
    pub cost_total: String,
}

impl BooksImportMigrationResponseStock {
    pub fn builder() -> BooksImportMigrationResponseStockBuilder {
        <BooksImportMigrationResponseStockBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseStockBuilder {
    movements: Option<i64>,
    cost_total: Option<String>,
}

impl BooksImportMigrationResponseStockBuilder {
    pub fn movements(mut self, value: i64) -> Self {
        self.movements = Some(value);
        self
    }

    pub fn cost_total(mut self, value: impl Into<String>) -> Self {
        self.cost_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationResponseStock`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movements`](BooksImportMigrationResponseStockBuilder::movements)
    /// - [`cost_total`](BooksImportMigrationResponseStockBuilder::cost_total)
    pub fn build(self) -> Result<BooksImportMigrationResponseStock, BuildError> {
        Ok(BooksImportMigrationResponseStock {
            movements: self
                .movements
                .ok_or_else(|| BuildError::missing_field("movements"))?,
            cost_total: self
                .cost_total
                .ok_or_else(|| BuildError::missing_field("cost_total"))?,
        })
    }
}
