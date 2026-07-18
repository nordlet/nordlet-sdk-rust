pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesSetOverridesResponse {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub source: PostV1ReferenceEuVatRatesSetOverridesResponseSource,
    #[serde(default)]
    pub notice: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceEuVatRatesSetOverridesResponseRowsItem>,
}

impl PostV1ReferenceEuVatRatesSetOverridesResponse {
    pub fn builder() -> PostV1ReferenceEuVatRatesSetOverridesResponseBuilder {
        <PostV1ReferenceEuVatRatesSetOverridesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesSetOverridesResponseBuilder {
    country_code: Option<String>,
    source: Option<PostV1ReferenceEuVatRatesSetOverridesResponseSource>,
    notice: Option<String>,
    rows: Option<Vec<PostV1ReferenceEuVatRatesSetOverridesResponseRowsItem>>,
}

impl PostV1ReferenceEuVatRatesSetOverridesResponseBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn source(mut self, value: PostV1ReferenceEuVatRatesSetOverridesResponseSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn notice(mut self, value: impl Into<String>) -> Self {
        self.notice = Some(value.into());
        self
    }

    pub fn rows(
        mut self,
        value: Vec<PostV1ReferenceEuVatRatesSetOverridesResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesSetOverridesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1ReferenceEuVatRatesSetOverridesResponseBuilder::country_code)
    /// - [`source`](PostV1ReferenceEuVatRatesSetOverridesResponseBuilder::source)
    /// - [`notice`](PostV1ReferenceEuVatRatesSetOverridesResponseBuilder::notice)
    /// - [`rows`](PostV1ReferenceEuVatRatesSetOverridesResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesSetOverridesResponse, BuildError> {
        Ok(PostV1ReferenceEuVatRatesSetOverridesResponse {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            notice: self
                .notice
                .ok_or_else(|| BuildError::missing_field("notice"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
