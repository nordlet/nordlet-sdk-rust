pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsEn {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsEn {
    pub fn builder() -> PostV1LedgerAccountsListResponseRowsItemTranslationsEnBuilder {
        <PostV1LedgerAccountsListResponseRowsItemTranslationsEnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsEnBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsEnBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsListResponseRowsItemTranslationsEn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsListResponseRowsItemTranslationsEnBuilder::name)
    pub fn build(
        self,
    ) -> Result<PostV1LedgerAccountsListResponseRowsItemTranslationsEn, BuildError> {
        Ok(PostV1LedgerAccountsListResponseRowsItemTranslationsEn {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
