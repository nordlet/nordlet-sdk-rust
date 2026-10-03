pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlCit8GenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsPlCit8GenerateRequest {
    pub fn builder() -> PostV1DeclarationsPlCit8GenerateRequestBuilder {
        <PostV1DeclarationsPlCit8GenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlCit8GenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsPlCit8GenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlCit8GenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlCit8GenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsPlCit8GenerateRequest, BuildError> {
        Ok(PostV1DeclarationsPlCit8GenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
