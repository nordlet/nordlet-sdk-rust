pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DownloadCalendarRequest {
    #[serde(default)]
    pub key: String,
}

impl DownloadCalendarRequest {
    pub fn builder() -> DownloadCalendarRequestBuilder {
        <DownloadCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DownloadCalendarRequestBuilder {
    key: Option<String>,
}

impl DownloadCalendarRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DownloadCalendarRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](DownloadCalendarRequestBuilder::key)
    pub fn build(self) -> Result<DownloadCalendarRequest, BuildError> {
        Ok(DownloadCalendarRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
