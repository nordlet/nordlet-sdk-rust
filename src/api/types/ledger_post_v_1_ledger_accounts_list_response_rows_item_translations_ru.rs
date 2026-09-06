pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsRu {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsRu {
    pub fn builder() -> PostV1LedgerAccountsListResponseRowsItemTranslationsRuBuilder {
        <PostV1LedgerAccountsListResponseRowsItemTranslationsRuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsRuBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsRuBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsListResponseRowsItemTranslationsRu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsListResponseRowsItemTranslationsRuBuilder::name)
    pub fn build(
        self,
    ) -> Result<PostV1LedgerAccountsListResponseRowsItemTranslationsRu, BuildError> {
        Ok(PostV1LedgerAccountsListResponseRowsItemTranslationsRu {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
