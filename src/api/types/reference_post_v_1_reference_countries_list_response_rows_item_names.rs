pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCountriesListResponseRowsItemNames {
    #[serde(default)]
    pub lt: String,
    #[serde(default)]
    pub en: String,
    #[serde(default)]
    pub ru: String,
}

impl PostV1ReferenceCountriesListResponseRowsItemNames {
    pub fn builder() -> PostV1ReferenceCountriesListResponseRowsItemNamesBuilder {
        <PostV1ReferenceCountriesListResponseRowsItemNamesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCountriesListResponseRowsItemNamesBuilder {
    lt: Option<String>,
    en: Option<String>,
    ru: Option<String>,
}

impl PostV1ReferenceCountriesListResponseRowsItemNamesBuilder {
    pub fn lt(mut self, value: impl Into<String>) -> Self {
        self.lt = Some(value.into());
        self
    }

    pub fn en(mut self, value: impl Into<String>) -> Self {
        self.en = Some(value.into());
        self
    }

    pub fn ru(mut self, value: impl Into<String>) -> Self {
        self.ru = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCountriesListResponseRowsItemNames`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lt`](PostV1ReferenceCountriesListResponseRowsItemNamesBuilder::lt)
    /// - [`en`](PostV1ReferenceCountriesListResponseRowsItemNamesBuilder::en)
    /// - [`ru`](PostV1ReferenceCountriesListResponseRowsItemNamesBuilder::ru)
    pub fn build(self) -> Result<PostV1ReferenceCountriesListResponseRowsItemNames, BuildError> {
        Ok(PostV1ReferenceCountriesListResponseRowsItemNames {
            lt: self.lt.ok_or_else(|| BuildError::missing_field("lt"))?,
            en: self.en.ok_or_else(|| BuildError::missing_field("en"))?,
            ru: self.ru.ok_or_else(|| BuildError::missing_field("ru"))?,
        })
    }
}
