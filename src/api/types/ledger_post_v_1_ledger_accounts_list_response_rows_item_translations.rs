pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<PostV1LedgerAccountsListResponseRowsItemTranslationsLt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<PostV1LedgerAccountsListResponseRowsItemTranslationsEn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru: Option<PostV1LedgerAccountsListResponseRowsItemTranslationsRu>,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslations {
    pub fn builder() -> PostV1LedgerAccountsListResponseRowsItemTranslationsBuilder {
        <PostV1LedgerAccountsListResponseRowsItemTranslationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsListResponseRowsItemTranslationsBuilder {
    lt: Option<PostV1LedgerAccountsListResponseRowsItemTranslationsLt>,
    en: Option<PostV1LedgerAccountsListResponseRowsItemTranslationsEn>,
    ru: Option<PostV1LedgerAccountsListResponseRowsItemTranslationsRu>,
}

impl PostV1LedgerAccountsListResponseRowsItemTranslationsBuilder {
    pub fn lt(mut self, value: PostV1LedgerAccountsListResponseRowsItemTranslationsLt) -> Self {
        self.lt = Some(value);
        self
    }

    pub fn en(mut self, value: PostV1LedgerAccountsListResponseRowsItemTranslationsEn) -> Self {
        self.en = Some(value);
        self
    }

    pub fn ru(mut self, value: PostV1LedgerAccountsListResponseRowsItemTranslationsRu) -> Self {
        self.ru = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsListResponseRowsItemTranslations`].
    pub fn build(self) -> Result<PostV1LedgerAccountsListResponseRowsItemTranslations, BuildError> {
        Ok(PostV1LedgerAccountsListResponseRowsItemTranslations {
            lt: self.lt,
            en: self.en,
            ru: self.ru,
        })
    }
}
