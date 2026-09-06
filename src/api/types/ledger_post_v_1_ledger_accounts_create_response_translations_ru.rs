pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateResponseTranslationsRu {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsCreateResponseTranslationsRu {
    pub fn builder() -> PostV1LedgerAccountsCreateResponseTranslationsRuBuilder {
        <PostV1LedgerAccountsCreateResponseTranslationsRuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateResponseTranslationsRuBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsCreateResponseTranslationsRuBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateResponseTranslationsRu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsCreateResponseTranslationsRuBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateResponseTranslationsRu, BuildError> {
        Ok(PostV1LedgerAccountsCreateResponseTranslationsRu {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
