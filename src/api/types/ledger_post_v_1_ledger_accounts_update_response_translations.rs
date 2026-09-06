pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateResponseTranslations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<PostV1LedgerAccountsUpdateResponseTranslationsLt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<PostV1LedgerAccountsUpdateResponseTranslationsEn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru: Option<PostV1LedgerAccountsUpdateResponseTranslationsRu>,
}

impl PostV1LedgerAccountsUpdateResponseTranslations {
    pub fn builder() -> PostV1LedgerAccountsUpdateResponseTranslationsBuilder {
        <PostV1LedgerAccountsUpdateResponseTranslationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsBuilder {
    lt: Option<PostV1LedgerAccountsUpdateResponseTranslationsLt>,
    en: Option<PostV1LedgerAccountsUpdateResponseTranslationsEn>,
    ru: Option<PostV1LedgerAccountsUpdateResponseTranslationsRu>,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsBuilder {
    pub fn lt(mut self, value: PostV1LedgerAccountsUpdateResponseTranslationsLt) -> Self {
        self.lt = Some(value);
        self
    }

    pub fn en(mut self, value: PostV1LedgerAccountsUpdateResponseTranslationsEn) -> Self {
        self.en = Some(value);
        self
    }

    pub fn ru(mut self, value: PostV1LedgerAccountsUpdateResponseTranslationsRu) -> Self {
        self.ru = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateResponseTranslations`].
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateResponseTranslations, BuildError> {
        Ok(PostV1LedgerAccountsUpdateResponseTranslations {
            lt: self.lt,
            en: self.en,
            ru: self.ru,
        })
    }
}
