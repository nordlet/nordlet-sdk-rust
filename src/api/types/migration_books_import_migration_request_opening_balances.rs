pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationRequestOpeningBalances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "balancingAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_account_code: Option<String>,
    #[serde(default)]
    pub entries: Vec<BooksImportMigrationRequestOpeningBalancesEntriesItem>,
}

impl BooksImportMigrationRequestOpeningBalances {
    pub fn builder() -> BooksImportMigrationRequestOpeningBalancesBuilder {
        <BooksImportMigrationRequestOpeningBalancesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationRequestOpeningBalancesBuilder {
    date: Option<NaiveDate>,
    balancing_account_code: Option<String>,
    entries: Option<Vec<BooksImportMigrationRequestOpeningBalancesEntriesItem>>,
}

impl BooksImportMigrationRequestOpeningBalancesBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn balancing_account_code(mut self, value: impl Into<String>) -> Self {
        self.balancing_account_code = Some(value.into());
        self
    }

    pub fn entries(
        mut self,
        value: Vec<BooksImportMigrationRequestOpeningBalancesEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationRequestOpeningBalances`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entries`](BooksImportMigrationRequestOpeningBalancesBuilder::entries)
    pub fn build(self) -> Result<BooksImportMigrationRequestOpeningBalances, BuildError> {
        Ok(BooksImportMigrationRequestOpeningBalances {
            date: self.date,
            balancing_account_code: self.balancing_account_code,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
