pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsCreateRequestTranslationsLt {
    #[serde(default)]
    pub name: String,
}

impl PostV1LedgerAccountsCreateRequestTranslationsLt {
    pub fn builder() -> PostV1LedgerAccountsCreateRequestTranslationsLtBuilder {
        <PostV1LedgerAccountsCreateRequestTranslationsLtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsCreateRequestTranslationsLtBuilder {
    name: Option<String>,
}

impl PostV1LedgerAccountsCreateRequestTranslationsLtBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsCreateRequestTranslationsLt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerAccountsCreateRequestTranslationsLtBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerAccountsCreateRequestTranslationsLt, BuildError> {
        Ok(PostV1LedgerAccountsCreateRequestTranslationsLt {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
