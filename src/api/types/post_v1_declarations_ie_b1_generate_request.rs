pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeB1GenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsIeB1GenerateRequest {
    pub fn builder() -> PostV1DeclarationsIeB1GenerateRequestBuilder {
        <PostV1DeclarationsIeB1GenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeB1GenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsIeB1GenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeB1GenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsIeB1GenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsIeB1GenerateRequest, BuildError> {
        Ok(PostV1DeclarationsIeB1GenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
