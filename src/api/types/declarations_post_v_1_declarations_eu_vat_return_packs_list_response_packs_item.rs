pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuVatReturnPacksListResponsePacksItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "formKey")]
    #[serde(default)]
    pub form_key: String,
    #[serde(rename = "formName")]
    #[serde(default)]
    pub form_name: String,
    pub frequency: PostV1DeclarationsEuVatReturnPacksListResponsePacksItemFrequency,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsEuVatReturnPacksListResponsePacksItem {
    pub fn builder() -> PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder {
        <PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder {
    country_code: Option<String>,
    form_key: Option<String>,
    form_name: Option<String>,
    frequency: Option<PostV1DeclarationsEuVatReturnPacksListResponsePacksItemFrequency>,
    source: Option<String>,
}

impl PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn form_key(mut self, value: impl Into<String>) -> Self {
        self.form_key = Some(value.into());
        self
    }

    pub fn form_name(mut self, value: impl Into<String>) -> Self {
        self.form_name = Some(value.into());
        self
    }

    pub fn frequency(
        mut self,
        value: PostV1DeclarationsEuVatReturnPacksListResponsePacksItemFrequency,
    ) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuVatReturnPacksListResponsePacksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder::country_code)
    /// - [`form_key`](PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder::form_key)
    /// - [`form_name`](PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder::form_name)
    /// - [`frequency`](PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder::frequency)
    /// - [`source`](PostV1DeclarationsEuVatReturnPacksListResponsePacksItemBuilder::source)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuVatReturnPacksListResponsePacksItem, BuildError> {
        Ok(PostV1DeclarationsEuVatReturnPacksListResponsePacksItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            form_key: self
                .form_key
                .ok_or_else(|| BuildError::missing_field("form_key"))?,
            form_name: self
                .form_name
                .ok_or_else(|| BuildError::missing_field("form_name"))?,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
