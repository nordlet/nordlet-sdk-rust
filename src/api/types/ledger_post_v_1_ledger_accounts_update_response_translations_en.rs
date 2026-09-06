pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsEn {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsEn {
    pub fn builder() -> PostV1LedgerAccountsUpdateResponseTranslationsEnBuilder {
        <PostV1LedgerAccountsUpdateResponseTranslationsEnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsEnBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsEnBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateResponseTranslationsEn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsUpdateResponseTranslationsEnBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateResponseTranslationsEn, BuildError> {
        Ok(PostV1LedgerAccountsUpdateResponseTranslationsEn {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
