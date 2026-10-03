pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarSubmitRequest {
    #[serde(default)]
    pub key: String,
}

impl PostV1CalendarSubmitRequest {
    pub fn builder() -> PostV1CalendarSubmitRequestBuilder {
        <PostV1CalendarSubmitRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarSubmitRequestBuilder {
    key: Option<String>,
}

impl PostV1CalendarSubmitRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CalendarSubmitRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1CalendarSubmitRequestBuilder::key)
    pub fn build(self) -> Result<PostV1CalendarSubmitRequest, BuildError> {
        Ok(PostV1CalendarSubmitRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
