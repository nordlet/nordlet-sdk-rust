pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsCreateResponseTranslationsValue {
    pub fn builder() -> PostV1LedgerAccountsCreateResponseTranslationsValueBuilder {
        <PostV1LedgerAccountsCreateResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateResponseTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsCreateResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsCreateResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateResponseTranslationsValue, BuildError> {
        Ok(PostV1LedgerAccountsCreateResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
