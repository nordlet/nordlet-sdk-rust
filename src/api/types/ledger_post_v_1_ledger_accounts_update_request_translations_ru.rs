pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsRu {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsRu {
    pub fn builder() -> PostV1LedgerAccountsUpdateRequestTranslationsRuBuilder {
        <PostV1LedgerAccountsUpdateRequestTranslationsRuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsRuBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsRuBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateRequestTranslationsRu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsUpdateRequestTranslationsRuBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateRequestTranslationsRu, BuildError> {
        Ok(PostV1LedgerAccountsUpdateRequestTranslationsRu {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
