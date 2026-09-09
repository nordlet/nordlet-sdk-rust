pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CalendarDeleteResponse {
    #[serde(default)]
    pub key: String,
}

impl PostV1CalendarDeleteResponse {
    pub fn builder() -> PostV1CalendarDeleteResponseBuilder {
        <PostV1CalendarDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CalendarDeleteResponseBuilder {
    key: Option<String>,
}

impl PostV1CalendarDeleteResponseBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CalendarDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1CalendarDeleteResponseBuilder::key)
    pub fn build(self) -> Result<PostV1CalendarDeleteResponse, BuildError> {
        Ok(PostV1CalendarDeleteResponse {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
