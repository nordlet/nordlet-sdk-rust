pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsImportBankResponse {
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub skipped: i64,
    #[serde(default)]
    pub duplicates: Vec<TransactionsImportBankResponseDuplicatesItem>,
}

impl TransactionsImportBankResponse {
    pub fn builder() -> TransactionsImportBankResponseBuilder {
        <TransactionsImportBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsImportBankResponseBuilder {
    imported: Option<i64>,
    skipped: Option<i64>,
    duplicates: Option<Vec<TransactionsImportBankResponseDuplicatesItem>>,
}

impl TransactionsImportBankResponseBuilder {
    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn duplicates(mut self, value: Vec<TransactionsImportBankResponseDuplicatesItem>) -> Self {
        self.duplicates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsImportBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`imported`](TransactionsImportBankResponseBuilder::imported)
    /// - [`skipped`](TransactionsImportBankResponseBuilder::skipped)
    /// - [`duplicates`](TransactionsImportBankResponseBuilder::duplicates)
    pub fn build(self) -> Result<TransactionsImportBankResponse, BuildError> {
        Ok(TransactionsImportBankResponse {
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
            duplicates: self
                .duplicates
                .ok_or_else(|| BuildError::missing_field("duplicates"))?,
        })
    }
}
