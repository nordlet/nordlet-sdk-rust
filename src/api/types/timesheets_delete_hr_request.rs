pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimesheetsDeleteHrRequest {
    #[serde(default)]
    pub id: String,
}

impl TimesheetsDeleteHrRequest {
    pub fn builder() -> TimesheetsDeleteHrRequestBuilder {
        <TimesheetsDeleteHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsDeleteHrRequestBuilder {
    id: Option<String>,
}

impl TimesheetsDeleteHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsDeleteHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimesheetsDeleteHrRequestBuilder::id)
    pub fn build(self) -> Result<TimesheetsDeleteHrRequest, BuildError> {
        Ok(TimesheetsDeleteHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
