pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateRequestTranslationsEn {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsCreateRequestTranslationsEn {
    pub fn builder() -> PostV1LedgerAccountsCreateRequestTranslationsEnBuilder {
        <PostV1LedgerAccountsCreateRequestTranslationsEnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateRequestTranslationsEnBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsCreateRequestTranslationsEnBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateRequestTranslationsEn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsCreateRequestTranslationsEnBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateRequestTranslationsEn, BuildError> {
        Ok(PostV1LedgerAccountsCreateRequestTranslationsEn {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
