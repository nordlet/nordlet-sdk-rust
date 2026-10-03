pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateRequestTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsCreateRequestTranslationsValue {
    pub fn builder() -> PostV1LedgerAccountsCreateRequestTranslationsValueBuilder {
        <PostV1LedgerAccountsCreateRequestTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateRequestTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsCreateRequestTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateRequestTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsCreateRequestTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateRequestTranslationsValue, BuildError> {
        Ok(PostV1LedgerAccountsCreateRequestTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
