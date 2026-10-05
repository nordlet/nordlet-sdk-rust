pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LocaleSetAccountRequest {
    pub locale: LocaleSetAccountRequestLocale,
}

impl LocaleSetAccountRequest {
    pub fn builder() -> LocaleSetAccountRequestBuilder {
        <LocaleSetAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LocaleSetAccountRequestBuilder {
    locale: Option<LocaleSetAccountRequestLocale>,
}

impl LocaleSetAccountRequestBuilder {
    pub fn locale(mut self, value: LocaleSetAccountRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LocaleSetAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`locale`](LocaleSetAccountRequestBuilder::locale)
    pub fn build(self) -> Result<LocaleSetAccountRequest, BuildError> {
        Ok(LocaleSetAccountRequest {
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
        })
    }
}
