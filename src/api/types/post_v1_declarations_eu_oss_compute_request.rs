pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuOssComputeRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub quarter: i64,
}

impl PostV1DeclarationsEuOssComputeRequest {
    pub fn builder() -> PostV1DeclarationsEuOssComputeRequestBuilder {
        <PostV1DeclarationsEuOssComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuOssComputeRequestBuilder {
    year: Option<i64>,
    quarter: Option<i64>,
}

impl PostV1DeclarationsEuOssComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: i64) -> Self {
        self.quarter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuOssComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsEuOssComputeRequestBuilder::year)
    /// - [`quarter`](PostV1DeclarationsEuOssComputeRequestBuilder::quarter)
    pub fn build(self) -> Result<PostV1DeclarationsEuOssComputeRequest, BuildError> {
        Ok(PostV1DeclarationsEuOssComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
        })
    }
}
