pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlPit11GenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsPlPit11GenerateRequest {
    pub fn builder() -> PostV1DeclarationsPlPit11GenerateRequestBuilder {
        <PostV1DeclarationsPlPit11GenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlPit11GenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsPlPit11GenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlPit11GenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlPit11GenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsPlPit11GenerateRequest, BuildError> {
        Ok(PostV1DeclarationsPlPit11GenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
