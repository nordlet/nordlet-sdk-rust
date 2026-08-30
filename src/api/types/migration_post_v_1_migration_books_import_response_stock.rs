pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponseStock {
    #[serde(default)]
    pub movements: i64,
    #[serde(rename = "costTotal")]
    #[serde(default)]
    pub cost_total: String,
}

impl PostV1MigrationBooksImportResponseStock {
    pub fn builder() -> PostV1MigrationBooksImportResponseStockBuilder {
        <PostV1MigrationBooksImportResponseStockBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseStockBuilder {
    movements: Option<i64>,
    cost_total: Option<String>,
}

impl PostV1MigrationBooksImportResponseStockBuilder {
    pub fn movements(mut self, value: i64) -> Self {
        self.movements = Some(value);
        self
    }

    pub fn cost_total(mut self, value: impl Into<String>) -> Self {
        self.cost_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponseStock`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movements`](PostV1MigrationBooksImportResponseStockBuilder::movements)
    /// - [`cost_total`](PostV1MigrationBooksImportResponseStockBuilder::cost_total)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponseStock, BuildError> {
        Ok(PostV1MigrationBooksImportResponseStock {
            movements: self
                .movements
                .ok_or_else(|| BuildError::missing_field("movements"))?,
            cost_total: self
                .cost_total
                .ok_or_else(|| BuildError::missing_field("cost_total"))?,
        })
    }
}
