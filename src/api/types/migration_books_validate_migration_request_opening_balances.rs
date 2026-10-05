pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationRequestOpeningBalances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "balancingAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_account_code: Option<String>,
    #[serde(default)]
    pub entries: Vec<BooksValidateMigrationRequestOpeningBalancesEntriesItem>,
}

impl BooksValidateMigrationRequestOpeningBalances {
    pub fn builder() -> BooksValidateMigrationRequestOpeningBalancesBuilder {
        <BooksValidateMigrationRequestOpeningBalancesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationRequestOpeningBalancesBuilder {
    date: Option<NaiveDate>,
    balancing_account_code: Option<String>,
    entries: Option<Vec<BooksValidateMigrationRequestOpeningBalancesEntriesItem>>,
}

impl BooksValidateMigrationRequestOpeningBalancesBuilder {
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
        value: Vec<BooksValidateMigrationRequestOpeningBalancesEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationRequestOpeningBalances`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entries`](BooksValidateMigrationRequestOpeningBalancesBuilder::entries)
    pub fn build(self) -> Result<BooksValidateMigrationRequestOpeningBalances, BuildError> {
        Ok(BooksValidateMigrationRequestOpeningBalances {
            date: self.date,
            balancing_account_code: self.balancing_account_code,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
