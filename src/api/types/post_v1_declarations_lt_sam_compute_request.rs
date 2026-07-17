pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSamComputeRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1DeclarationsLtSamComputeRequest {
    pub fn builder() -> PostV1DeclarationsLtSamComputeRequestBuilder {
        <PostV1DeclarationsLtSamComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSamComputeRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsLtSamComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSamComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtSamComputeRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsLtSamComputeRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsLtSamComputeRequest, BuildError> {
        Ok(PostV1DeclarationsLtSamComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
