pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCountriesListResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "isEu")]
    #[serde(default)]
    pub is_eu: bool,
    #[serde(rename = "isEea")]
    #[serde(default)]
    pub is_eea: bool,
    #[serde(default)]
    pub names: PostV1ReferenceCountriesListResponseRowsItemNames,
}

impl PostV1ReferenceCountriesListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceCountriesListResponseRowsItemBuilder {
        <PostV1ReferenceCountriesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCountriesListResponseRowsItemBuilder {
    code: Option<String>,
    is_eu: Option<bool>,
    is_eea: Option<bool>,
    names: Option<PostV1ReferenceCountriesListResponseRowsItemNames>,
}

impl PostV1ReferenceCountriesListResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn is_eu(mut self, value: bool) -> Self {
        self.is_eu = Some(value);
        self
    }

    pub fn is_eea(mut self, value: bool) -> Self {
        self.is_eea = Some(value);
        self
    }

    pub fn names(mut self, value: PostV1ReferenceCountriesListResponseRowsItemNames) -> Self {
        self.names = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCountriesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReferenceCountriesListResponseRowsItemBuilder::code)
    /// - [`is_eu`](PostV1ReferenceCountriesListResponseRowsItemBuilder::is_eu)
    /// - [`is_eea`](PostV1ReferenceCountriesListResponseRowsItemBuilder::is_eea)
    /// - [`names`](PostV1ReferenceCountriesListResponseRowsItemBuilder::names)
    pub fn build(self) -> Result<PostV1ReferenceCountriesListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceCountriesListResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            is_eu: self
                .is_eu
                .ok_or_else(|| BuildError::missing_field("is_eu"))?,
            is_eea: self
                .is_eea
                .ok_or_else(|| BuildError::missing_field("is_eea"))?,
            names: self
                .names
                .ok_or_else(|| BuildError::missing_field("names"))?,
        })
    }
}
