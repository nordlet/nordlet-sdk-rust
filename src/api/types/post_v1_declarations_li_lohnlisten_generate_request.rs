pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLiLohnlistenGenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsLiLohnlistenGenerateRequest {
    pub fn builder() -> PostV1DeclarationsLiLohnlistenGenerateRequestBuilder {
        <PostV1DeclarationsLiLohnlistenGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLiLohnlistenGenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsLiLohnlistenGenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLiLohnlistenGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLiLohnlistenGenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsLiLohnlistenGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsLiLohnlistenGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
