pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdGetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl PostV1DeclarationsEuSmeThresholdGetRequest {
    pub fn builder() -> PostV1DeclarationsEuSmeThresholdGetRequestBuilder {
        <PostV1DeclarationsEuSmeThresholdGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdGetRequestBuilder {
    date: Option<String>,
}

impl PostV1DeclarationsEuSmeThresholdGetRequestBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdGetRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsEuSmeThresholdGetRequest, BuildError> {
        Ok(PostV1DeclarationsEuSmeThresholdGetRequest { date: self.date })
    }
}
