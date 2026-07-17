pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIntrastatObligationRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsLtIntrastatObligationRequest {
    pub fn builder() -> PostV1DeclarationsLtIntrastatObligationRequestBuilder {
        <PostV1DeclarationsLtIntrastatObligationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIntrastatObligationRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsLtIntrastatObligationRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIntrastatObligationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtIntrastatObligationRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsLtIntrastatObligationRequest, BuildError> {
        Ok(PostV1DeclarationsLtIntrastatObligationRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
