pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarGetRequest {
    #[serde(default)]
    pub key: String,
}

impl PostV1CalendarGetRequest {
    pub fn builder() -> PostV1CalendarGetRequestBuilder {
        <PostV1CalendarGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarGetRequestBuilder {
    key: Option<String>,
}

impl PostV1CalendarGetRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CalendarGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1CalendarGetRequestBuilder::key)
    pub fn build(self) -> Result<PostV1CalendarGetRequest, BuildError> {
        Ok(PostV1CalendarGetRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
