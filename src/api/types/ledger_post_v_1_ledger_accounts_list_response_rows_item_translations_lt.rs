pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsLt {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsLt {
    pub fn builder() -> PostV1LedgerAccountsListResponseRowsItemTranslationsLtBuilder {
        <PostV1LedgerAccountsListResponseRowsItemTranslationsLtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsLtBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsLtBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsListResponseRowsItemTranslationsLt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsListResponseRowsItemTranslationsLtBuilder::name)
    pub fn build(
        self,
    ) -> Result<PostV1LedgerAccountsListResponseRowsItemTranslationsLt, BuildError> {
        Ok(PostV1LedgerAccountsListResponseRowsItemTranslationsLt {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
