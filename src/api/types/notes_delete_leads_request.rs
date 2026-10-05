pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotesDeleteLeadsRequest {
    #[serde(default)]
    pub id: String,
}

impl NotesDeleteLeadsRequest {
    pub fn builder() -> NotesDeleteLeadsRequestBuilder {
        <NotesDeleteLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotesDeleteLeadsRequestBuilder {
    id: Option<String>,
}

impl NotesDeleteLeadsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotesDeleteLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](NotesDeleteLeadsRequestBuilder::id)
    pub fn build(self) -> Result<NotesDeleteLeadsRequest, BuildError> {
        Ok(NotesDeleteLeadsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
