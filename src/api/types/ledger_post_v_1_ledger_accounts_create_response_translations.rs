pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateResponseTranslations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<PostV1LedgerAccountsCreateResponseTranslationsLt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<PostV1LedgerAccountsCreateResponseTranslationsEn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru: Option<PostV1LedgerAccountsCreateResponseTranslationsRu>,
}

impl PostV1LedgerAccountsCreateResponseTranslations {
    pub fn builder() -> PostV1LedgerAccountsCreateResponseTranslationsBuilder {
        <PostV1LedgerAccountsCreateResponseTranslationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateResponseTranslationsBuilder {
    lt: Option<PostV1LedgerAccountsCreateResponseTranslationsLt>,
    en: Option<PostV1LedgerAccountsCreateResponseTranslationsEn>,
    ru: Option<PostV1LedgerAccountsCreateResponseTranslationsRu>,
}

impl PostV1LedgerAccountsCreateResponseTranslationsBuilder {
    pub fn lt(mut self, value: PostV1LedgerAccountsCreateResponseTranslationsLt) -> Self {
        self.lt = Some(value);
        self
    }

    pub fn en(mut self, value: PostV1LedgerAccountsCreateResponseTranslationsEn) -> Self {
        self.en = Some(value);
        self
    }

    pub fn ru(mut self, value: PostV1LedgerAccountsCreateResponseTranslationsRu) -> Self {
        self.ru = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateResponseTranslations`].
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateResponseTranslations, BuildError> {
        Ok(PostV1LedgerAccountsCreateResponseTranslations {
            lt: self.lt,
            en: self.en,
            ru: self.ru,
        })
    }
}
