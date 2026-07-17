pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesListRequest {
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl PostV1ReferenceEuVatRatesListRequest {
    pub fn builder() -> PostV1ReferenceEuVatRatesListRequestBuilder {
        <PostV1ReferenceEuVatRatesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesListRequestBuilder {
    country_code: Option<String>,
    date: Option<String>,
}

impl PostV1ReferenceEuVatRatesListRequestBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesListRequest, BuildError> {
        Ok(PostV1ReferenceEuVatRatesListRequest {
            country_code: self.country_code,
            date: self.date,
        })
    }
}
