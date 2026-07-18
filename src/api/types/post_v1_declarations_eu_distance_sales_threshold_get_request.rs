pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuDistanceSalesThresholdGetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl PostV1DeclarationsEuDistanceSalesThresholdGetRequest {
    pub fn builder() -> PostV1DeclarationsEuDistanceSalesThresholdGetRequestBuilder {
        <PostV1DeclarationsEuDistanceSalesThresholdGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuDistanceSalesThresholdGetRequestBuilder {
    date: Option<String>,
}

impl PostV1DeclarationsEuDistanceSalesThresholdGetRequestBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuDistanceSalesThresholdGetRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsEuDistanceSalesThresholdGetRequest, BuildError> {
        Ok(PostV1DeclarationsEuDistanceSalesThresholdGetRequest { date: self.date })
    }
}
