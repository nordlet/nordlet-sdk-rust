pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportRequestOpeningBalances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "balancingAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_account_code: Option<String>,
    #[serde(default)]
    pub entries: Vec<PostV1MigrationBooksImportRequestOpeningBalancesEntriesItem>,
}

impl PostV1MigrationBooksImportRequestOpeningBalances {
    pub fn builder() -> PostV1MigrationBooksImportRequestOpeningBalancesBuilder {
        <PostV1MigrationBooksImportRequestOpeningBalancesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportRequestOpeningBalancesBuilder {
    date: Option<String>,
    balancing_account_code: Option<String>,
    entries: Option<Vec<PostV1MigrationBooksImportRequestOpeningBalancesEntriesItem>>,
}

impl PostV1MigrationBooksImportRequestOpeningBalancesBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn balancing_account_code(mut self, value: impl Into<String>) -> Self {
        self.balancing_account_code = Some(value.into());
        self
    }

    pub fn entries(
        mut self,
        value: Vec<PostV1MigrationBooksImportRequestOpeningBalancesEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportRequestOpeningBalances`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entries`](PostV1MigrationBooksImportRequestOpeningBalancesBuilder::entries)
    pub fn build(self) -> Result<PostV1MigrationBooksImportRequestOpeningBalances, BuildError> {
        Ok(PostV1MigrationBooksImportRequestOpeningBalances {
            date: self.date,
            balancing_account_code: self.balancing_account_code,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
