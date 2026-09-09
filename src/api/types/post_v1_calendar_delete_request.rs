pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarDeleteRequest {
    #[serde(default)]
    pub key: String,
}

impl PostV1CalendarDeleteRequest {
    pub fn builder() -> PostV1CalendarDeleteRequestBuilder {
        <PostV1CalendarDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarDeleteRequestBuilder {
    key: Option<String>,
}

impl PostV1CalendarDeleteRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CalendarDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1CalendarDeleteRequestBuilder::key)
    pub fn build(self) -> Result<PostV1CalendarDeleteRequest, BuildError> {
        Ok(PostV1CalendarDeleteRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
