pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCalendarRequest {
    #[serde(default)]
    pub key: String,
}

impl GetCalendarRequest {
    pub fn builder() -> GetCalendarRequestBuilder {
        <GetCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCalendarRequestBuilder {
    key: Option<String>,
}

impl GetCalendarRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCalendarRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](GetCalendarRequestBuilder::key)
    pub fn build(self) -> Result<GetCalendarRequest, BuildError> {
        Ok(GetCalendarRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
