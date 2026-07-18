pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionComputeRequest {
    #[serde(rename = "asOfDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_date: Option<String>,
}

impl PostV1SalesRecognitionComputeRequest {
    pub fn builder() -> PostV1SalesRecognitionComputeRequestBuilder {
        <PostV1SalesRecognitionComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionComputeRequestBuilder {
    as_of_date: Option<String>,
}

impl PostV1SalesRecognitionComputeRequestBuilder {
    pub fn as_of_date(mut self, value: impl Into<String>) -> Self {
        self.as_of_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionComputeRequest`].
    pub fn build(self) -> Result<PostV1SalesRecognitionComputeRequest, BuildError> {
        Ok(PostV1SalesRecognitionComputeRequest {
            as_of_date: self.as_of_date,
        })
    }
}
