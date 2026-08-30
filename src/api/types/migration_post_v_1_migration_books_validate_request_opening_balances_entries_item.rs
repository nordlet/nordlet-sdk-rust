pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItem {
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItem {
    pub fn builder() -> PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItemBuilder {
        <PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItemBuilder {
    account_code: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
    description: Option<String>,
}

impl PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItemBuilder {
    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn debit(mut self, value: impl Into<String>) -> Self {
        self.debit = Some(value.into());
        self
    }

    pub fn credit(mut self, value: impl Into<String>) -> Self {
        self.credit = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_code`](PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItemBuilder::account_code)
    pub fn build(
        self,
    ) -> Result<PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItem, BuildError> {
        Ok(
            PostV1MigrationBooksValidateRequestOpeningBalancesEntriesItem {
                account_code: self
                    .account_code
                    .ok_or_else(|| BuildError::missing_field("account_code"))?,
                debit: self.debit,
                credit: self.credit,
                description: self.description,
            },
        )
    }
}
