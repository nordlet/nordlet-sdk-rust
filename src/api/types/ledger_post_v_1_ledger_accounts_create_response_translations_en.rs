pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateResponseTranslationsEn {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsCreateResponseTranslationsEn {
    pub fn builder() -> PostV1LedgerAccountsCreateResponseTranslationsEnBuilder {
        <PostV1LedgerAccountsCreateResponseTranslationsEnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateResponseTranslationsEnBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsCreateResponseTranslationsEnBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateResponseTranslationsEn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsCreateResponseTranslationsEnBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateResponseTranslationsEn, BuildError> {
        Ok(PostV1LedgerAccountsCreateResponseTranslationsEn {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
