pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteCalendarRequest {
    #[serde(default)]
    pub key: String,
}

impl DeleteCalendarRequest {
    pub fn builder() -> DeleteCalendarRequestBuilder {
        <DeleteCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteCalendarRequestBuilder {
    key: Option<String>,
}

impl DeleteCalendarRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteCalendarRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](DeleteCalendarRequestBuilder::key)
    pub fn build(self) -> Result<DeleteCalendarRequest, BuildError> {
        Ok(DeleteCalendarRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
