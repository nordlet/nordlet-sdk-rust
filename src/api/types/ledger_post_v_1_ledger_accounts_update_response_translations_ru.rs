pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsRu {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsRu {
    pub fn builder() -> PostV1LedgerAccountsUpdateResponseTranslationsRuBuilder {
        <PostV1LedgerAccountsUpdateResponseTranslationsRuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsRuBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsRuBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateResponseTranslationsRu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsUpdateResponseTranslationsRuBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateResponseTranslationsRu, BuildError> {
        Ok(PostV1LedgerAccountsUpdateResponseTranslationsRu {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
