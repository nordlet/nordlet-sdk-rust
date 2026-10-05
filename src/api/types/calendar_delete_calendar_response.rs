pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteCalendarResponse {
    #[serde(default)]
    pub key: String,
}

impl DeleteCalendarResponse {
    pub fn builder() -> DeleteCalendarResponseBuilder {
        <DeleteCalendarResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteCalendarResponseBuilder {
    key: Option<String>,
}

impl DeleteCalendarResponseBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteCalendarResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](DeleteCalendarResponseBuilder::key)
    pub fn build(self) -> Result<DeleteCalendarResponse, BuildError> {
        Ok(DeleteCalendarResponse {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
