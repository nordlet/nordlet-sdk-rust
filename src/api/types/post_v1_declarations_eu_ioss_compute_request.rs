pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuIossComputeRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1DeclarationsEuIossComputeRequest {
    pub fn builder() -> PostV1DeclarationsEuIossComputeRequestBuilder {
        <PostV1DeclarationsEuIossComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuIossComputeRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1DeclarationsEuIossComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuIossComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsEuIossComputeRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsEuIossComputeRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsEuIossComputeRequest, BuildError> {
        Ok(PostV1DeclarationsEuIossComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
