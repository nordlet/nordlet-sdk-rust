pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateRequestTranslations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<PostV1LedgerAccountsCreateRequestTranslationsLt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<PostV1LedgerAccountsCreateRequestTranslationsEn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru: Option<PostV1LedgerAccountsCreateRequestTranslationsRu>,
}

impl PostV1LedgerAccountsCreateRequestTranslations {
    pub fn builder() -> PostV1LedgerAccountsCreateRequestTranslationsBuilder {
        <PostV1LedgerAccountsCreateRequestTranslationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateRequestTranslationsBuilder {
    lt: Option<PostV1LedgerAccountsCreateRequestTranslationsLt>,
    en: Option<PostV1LedgerAccountsCreateRequestTranslationsEn>,
    ru: Option<PostV1LedgerAccountsCreateRequestTranslationsRu>,
}

impl PostV1LedgerAccountsCreateRequestTranslationsBuilder {
    pub fn lt(mut self, value: PostV1LedgerAccountsCreateRequestTranslationsLt) -> Self {
        self.lt = Some(value);
        self
    }

    pub fn en(mut self, value: PostV1LedgerAccountsCreateRequestTranslationsEn) -> Self {
        self.en = Some(value);
        self
    }

    pub fn ru(mut self, value: PostV1LedgerAccountsCreateRequestTranslationsRu) -> Self {
        self.ru = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateRequestTranslations`].
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateRequestTranslations, BuildError> {
        Ok(PostV1LedgerAccountsCreateRequestTranslations {
            lt: self.lt,
            en: self.en,
            ru: self.ru,
        })
    }
}
