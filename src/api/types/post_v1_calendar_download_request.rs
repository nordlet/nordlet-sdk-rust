pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarDownloadRequest {
    #[serde(default)]
    pub key: String,
}

impl PostV1CalendarDownloadRequest {
    pub fn builder() -> PostV1CalendarDownloadRequestBuilder {
        <PostV1CalendarDownloadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarDownloadRequestBuilder {
    key: Option<String>,
}

impl PostV1CalendarDownloadRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CalendarDownloadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1CalendarDownloadRequestBuilder::key)
    pub fn build(self) -> Result<PostV1CalendarDownloadRequest, BuildError> {
        Ok(PostV1CalendarDownloadRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
