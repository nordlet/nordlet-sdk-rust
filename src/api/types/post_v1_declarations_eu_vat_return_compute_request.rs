pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuVatReturnComputeRequest {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i64>,
}

impl PostV1DeclarationsEuVatReturnComputeRequest {
    pub fn builder() -> PostV1DeclarationsEuVatReturnComputeRequestBuilder {
        <PostV1DeclarationsEuVatReturnComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuVatReturnComputeRequestBuilder {
    country_code: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    months: Option<i64>,
}

impl PostV1DeclarationsEuVatReturnComputeRequestBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn months(mut self, value: i64) -> Self {
        self.months = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuVatReturnComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsEuVatReturnComputeRequestBuilder::country_code)
    /// - [`year`](PostV1DeclarationsEuVatReturnComputeRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsEuVatReturnComputeRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsEuVatReturnComputeRequest, BuildError> {
        Ok(PostV1DeclarationsEuVatReturnComputeRequest {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            months: self.months,
        })
    }
}
