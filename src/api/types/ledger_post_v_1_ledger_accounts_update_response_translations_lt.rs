pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsLt {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsLt {
    pub fn builder() -> PostV1LedgerAccountsUpdateResponseTranslationsLtBuilder {
        <PostV1LedgerAccountsUpdateResponseTranslationsLtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsUpdateResponseTranslationsLtBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsUpdateResponseTranslationsLtBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsUpdateResponseTranslationsLt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsUpdateResponseTranslationsLtBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsUpdateResponseTranslationsLt, BuildError> {
        Ok(PostV1LedgerAccountsUpdateResponseTranslationsLt {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
