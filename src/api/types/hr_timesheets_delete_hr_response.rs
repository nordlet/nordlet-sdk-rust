pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimesheetsDeleteHrResponse {
    #[serde(default)]
    pub id: String,
}

impl TimesheetsDeleteHrResponse {
    pub fn builder() -> TimesheetsDeleteHrResponseBuilder {
        <TimesheetsDeleteHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsDeleteHrResponseBuilder {
    id: Option<String>,
}

impl TimesheetsDeleteHrResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsDeleteHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimesheetsDeleteHrResponseBuilder::id)
    pub fn build(self) -> Result<TimesheetsDeleteHrResponse, BuildError> {
        Ok(TimesheetsDeleteHrResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
