pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtPln204ComputeRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsLtPln204ComputeRequest {
    pub fn builder() -> PostV1DeclarationsLtPln204ComputeRequestBuilder {
        <PostV1DeclarationsLtPln204ComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtPln204ComputeRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsLtPln204ComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtPln204ComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtPln204ComputeRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsLtPln204ComputeRequest, BuildError> {
        Ok(PostV1DeclarationsLtPln204ComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
