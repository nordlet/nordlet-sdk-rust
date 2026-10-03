pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsValue {
    pub fn builder() -> PostV1LedgerAccountsListResponseRowsItemTranslationsValueBuilder {
        <PostV1LedgerAccountsListResponseRowsItemTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsListResponseRowsItemTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsListResponseRowsItemTranslationsValueBuilder::name)
    pub fn build(
        self,
    ) -> Result<PostV1LedgerAccountsListResponseRowsItemTranslationsValue, BuildError> {
        Ok(PostV1LedgerAccountsListResponseRowsItemTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
