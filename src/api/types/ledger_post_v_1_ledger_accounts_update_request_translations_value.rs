pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsValue {
    pub fn builder() -> PostV1LedgerAccountsUpdateRequestTranslationsValueBuilder {
        <PostV1LedgerAccountsUpdateRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsUpdateRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateRequestTranslationsValue, BuildError> {
        Ok(PostV1LedgerAccountsUpdateRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
