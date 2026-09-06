pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateResponseTranslationsLt {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsCreateResponseTranslationsLt {
    pub fn builder() -> PostV1LedgerAccountsCreateResponseTranslationsLtBuilder {
        <PostV1LedgerAccountsCreateResponseTranslationsLtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateResponseTranslationsLtBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsCreateResponseTranslationsLtBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateResponseTranslationsLt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsCreateResponseTranslationsLtBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateResponseTranslationsLt, BuildError> {
        Ok(PostV1LedgerAccountsCreateResponseTranslationsLt {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
