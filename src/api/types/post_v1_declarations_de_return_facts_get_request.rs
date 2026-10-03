pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsGetRequest {
    #[serde(default)]
    pub year: i64,
}

impl PostV1DeclarationsDeReturnFactsGetRequest {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsGetRequestBuilder {
        <PostV1DeclarationsDeReturnFactsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsGetRequestBuilder {
    year: Option<i64>,
}

impl PostV1DeclarationsDeReturnFactsGetRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsDeReturnFactsGetRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsDeReturnFactsGetRequest, BuildError> {
        Ok(PostV1DeclarationsDeReturnFactsGetRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
