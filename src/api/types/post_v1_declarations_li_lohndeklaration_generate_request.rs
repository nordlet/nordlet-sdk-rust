pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLiLohndeklarationGenerateRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsLiLohndeklarationGenerateRequest {
    pub fn builder() -> PostV1DeclarationsLiLohndeklarationGenerateRequestBuilder {
        <PostV1DeclarationsLiLohndeklarationGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLiLohndeklarationGenerateRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsLiLohndeklarationGenerateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLiLohndeklarationGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLiLohndeklarationGenerateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsLiLohndeklarationGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsLiLohndeklarationGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
