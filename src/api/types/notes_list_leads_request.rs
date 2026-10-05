pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotesListLeadsRequest {
    #[serde(rename = "leadId")]
    #[serde(default)]
    pub lead_id: String,
}

impl NotesListLeadsRequest {
    pub fn builder() -> NotesListLeadsRequestBuilder {
        <NotesListLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotesListLeadsRequestBuilder {
    lead_id: Option<String>,
}

impl NotesListLeadsRequestBuilder {
    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotesListLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lead_id`](NotesListLeadsRequestBuilder::lead_id)
    pub fn build(self) -> Result<NotesListLeadsRequest, BuildError> {
        Ok(NotesListLeadsRequest {
            lead_id: self
                .lead_id
                .ok_or_else(|| BuildError::missing_field("lead_id"))?,
        })
    }
}
