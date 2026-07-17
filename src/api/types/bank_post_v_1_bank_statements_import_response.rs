pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankStatementsImportResponse {
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub skipped: i64,
    #[serde(default)]
    pub statements: Vec<PostV1BankStatementsImportResponseStatementsItem>,
}

impl PostV1BankStatementsImportResponse {
    pub fn builder() -> PostV1BankStatementsImportResponseBuilder {
        <PostV1BankStatementsImportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankStatementsImportResponseBuilder {
    imported: Option<i64>,
    skipped: Option<i64>,
    statements: Option<Vec<PostV1BankStatementsImportResponseStatementsItem>>,
}

impl PostV1BankStatementsImportResponseBuilder {
    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn statements(
        mut self,
        value: Vec<PostV1BankStatementsImportResponseStatementsItem>,
    ) -> Self {
        self.statements = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankStatementsImportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`imported`](PostV1BankStatementsImportResponseBuilder::imported)
    /// - [`skipped`](PostV1BankStatementsImportResponseBuilder::skipped)
    /// - [`statements`](PostV1BankStatementsImportResponseBuilder::statements)
    pub fn build(self) -> Result<PostV1BankStatementsImportResponse, BuildError> {
        Ok(PostV1BankStatementsImportResponse {
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
            statements: self
                .statements
                .ok_or_else(|| BuildError::missing_field("statements"))?,
        })
    }
}
