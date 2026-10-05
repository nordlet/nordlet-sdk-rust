pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitCalendarRequest {
    #[serde(default)]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend: Option<bool>,
}

impl SubmitCalendarRequest {
    pub fn builder() -> SubmitCalendarRequestBuilder {
        <SubmitCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitCalendarRequestBuilder {
    key: Option<String>,
    amend: Option<bool>,
}

impl SubmitCalendarRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn amend(mut self, value: bool) -> Self {
        self.amend = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitCalendarRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](SubmitCalendarRequestBuilder::key)
    pub fn build(self) -> Result<SubmitCalendarRequest, BuildError> {
        Ok(SubmitCalendarRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            amend: self.amend,
        })
    }
}
