pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsLt {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsLt {
    pub fn builder() -> PostV1LedgerAccountsUpdateRequestTranslationsLtBuilder {
        <PostV1LedgerAccountsUpdateRequestTranslationsLtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateRequestTranslationsLtBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsUpdateRequestTranslationsLtBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateRequestTranslationsLt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsUpdateRequestTranslationsLtBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateRequestTranslationsLt, BuildError> {
        Ok(PostV1LedgerAccountsUpdateRequestTranslationsLt {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
