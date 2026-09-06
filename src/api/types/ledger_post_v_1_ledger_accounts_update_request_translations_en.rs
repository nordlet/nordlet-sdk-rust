pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsEn {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsEn {
    pub fn builder() -> PostV1LedgerAccountsUpdateRequestTranslationsEnBuilder {
        <PostV1LedgerAccountsUpdateRequestTranslationsEnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsEnBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsEnBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateRequestTranslationsEn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsUpdateRequestTranslationsEnBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateRequestTranslationsEn, BuildError> {
        Ok(PostV1LedgerAccountsUpdateRequestTranslationsEn {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
