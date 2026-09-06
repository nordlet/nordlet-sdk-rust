pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateRequestTranslations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<PostV1LedgerAccountsUpdateRequestTranslationsLt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<PostV1LedgerAccountsUpdateRequestTranslationsEn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru: Option<PostV1LedgerAccountsUpdateRequestTranslationsRu>,
}

impl PostV1LedgerAccountsUpdateRequestTranslations {
    pub fn builder() -> PostV1LedgerAccountsUpdateRequestTranslationsBuilder {
        <PostV1LedgerAccountsUpdateRequestTranslationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsBuilder {
    lt: Option<PostV1LedgerAccountsUpdateRequestTranslationsLt>,
    en: Option<PostV1LedgerAccountsUpdateRequestTranslationsEn>,
    ru: Option<PostV1LedgerAccountsUpdateRequestTranslationsRu>,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsBuilder {
    pub fn lt(mut self, value: PostV1LedgerAccountsUpdateRequestTranslationsLt) -> Self {
        self.lt = Some(value);
        self
    }

    pub fn en(mut self, value: PostV1LedgerAccountsUpdateRequestTranslationsEn) -> Self {
        self.en = Some(value);
        self
    }

    pub fn ru(mut self, value: PostV1LedgerAccountsUpdateRequestTranslationsRu) -> Self {
        self.ru = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateRequestTranslations`].
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateRequestTranslations, BuildError> {
        Ok(PostV1LedgerAccountsUpdateRequestTranslations {
            lt: self.lt,
            en: self.en,
            ru: self.ru,
        })
    }
}
