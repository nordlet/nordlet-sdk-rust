pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsValue {
    pub fn builder() -> PostV1LedgerAccountsUpdateResponseTranslationsValueBuilder {
        <PostV1LedgerAccountsUpdateResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsUpdateResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateResponseTranslationsValue, BuildError> {
        Ok(PostV1LedgerAccountsUpdateResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
