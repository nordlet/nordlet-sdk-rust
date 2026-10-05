pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LocaleSetAccountResponse {
    #[serde(default)]
    pub locale: String,
    pub scope: LocaleSetAccountResponseScope,
}

impl LocaleSetAccountResponse {
    pub fn builder() -> LocaleSetAccountResponseBuilder {
        <LocaleSetAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LocaleSetAccountResponseBuilder {
    locale: Option<String>,
    scope: Option<LocaleSetAccountResponseScope>,
}

impl LocaleSetAccountResponseBuilder {
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn scope(mut self, value: LocaleSetAccountResponseScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LocaleSetAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`locale`](LocaleSetAccountResponseBuilder::locale)
    /// - [`scope`](LocaleSetAccountResponseBuilder::scope)
    pub fn build(self) -> Result<LocaleSetAccountResponse, BuildError> {
        Ok(LocaleSetAccountResponse {
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
