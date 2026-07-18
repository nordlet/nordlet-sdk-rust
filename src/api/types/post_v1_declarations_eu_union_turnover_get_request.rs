pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuUnionTurnoverGetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl PostV1DeclarationsEuUnionTurnoverGetRequest {
    pub fn builder() -> PostV1DeclarationsEuUnionTurnoverGetRequestBuilder {
        <PostV1DeclarationsEuUnionTurnoverGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuUnionTurnoverGetRequestBuilder {
    date: Option<String>,
}

impl PostV1DeclarationsEuUnionTurnoverGetRequestBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuUnionTurnoverGetRequest`].
    pub fn build(self) -> Result<PostV1DeclarationsEuUnionTurnoverGetRequest, BuildError> {
        Ok(PostV1DeclarationsEuUnionTurnoverGetRequest { date: self.date })
    }
}
