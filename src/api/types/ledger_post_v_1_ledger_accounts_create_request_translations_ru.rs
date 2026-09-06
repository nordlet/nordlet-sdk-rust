pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateRequestTranslationsRu {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsCreateRequestTranslationsRu {
    pub fn builder() -> PostV1LedgerAccountsCreateRequestTranslationsRuBuilder {
        <PostV1LedgerAccountsCreateRequestTranslationsRuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateRequestTranslationsRuBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsCreateRequestTranslationsRuBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateRequestTranslationsRu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsCreateRequestTranslationsRuBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateRequestTranslationsRu, BuildError> {
        Ok(PostV1LedgerAccountsCreateRequestTranslationsRu {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
