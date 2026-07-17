pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankTransactionsImportResponse {
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub skipped: i64,
}

impl PostV1BankTransactionsImportResponse {
    pub fn builder() -> PostV1BankTransactionsImportResponseBuilder {
        <PostV1BankTransactionsImportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsImportResponseBuilder {
    imported: Option<i64>,
    skipped: Option<i64>,
}

impl PostV1BankTransactionsImportResponseBuilder {
    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankTransactionsImportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`imported`](PostV1BankTransactionsImportResponseBuilder::imported)
    /// - [`skipped`](PostV1BankTransactionsImportResponseBuilder::skipped)
    pub fn build(self) -> Result<PostV1BankTransactionsImportResponse, BuildError> {
        Ok(PostV1BankTransactionsImportResponse {
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
        })
    }
}
