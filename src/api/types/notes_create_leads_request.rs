pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotesCreateLeadsRequest {
    #[serde(rename = "leadId")]
    #[serde(default)]
    pub lead_id: String,
    #[serde(default)]
    pub body: String,
}

impl NotesCreateLeadsRequest {
    pub fn builder() -> NotesCreateLeadsRequestBuilder {
        <NotesCreateLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotesCreateLeadsRequestBuilder {
    lead_id: Option<String>,
    body: Option<String>,
}

impl NotesCreateLeadsRequestBuilder {
    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotesCreateLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lead_id`](NotesCreateLeadsRequestBuilder::lead_id)
    /// - [`body`](NotesCreateLeadsRequestBuilder::body)
    pub fn build(self) -> Result<NotesCreateLeadsRequest, BuildError> {
        Ok(NotesCreateLeadsRequest {
            lead_id: self
                .lead_id
                .ok_or_else(|| BuildError::missing_field("lead_id"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
