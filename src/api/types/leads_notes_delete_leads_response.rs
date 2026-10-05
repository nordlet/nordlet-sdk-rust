pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotesDeleteLeadsResponse {
    #[serde(default)]
    pub id: String,
}

impl NotesDeleteLeadsResponse {
    pub fn builder() -> NotesDeleteLeadsResponseBuilder {
        <NotesDeleteLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotesDeleteLeadsResponseBuilder {
    id: Option<String>,
}

impl NotesDeleteLeadsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotesDeleteLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](NotesDeleteLeadsResponseBuilder::id)
    pub fn build(self) -> Result<NotesDeleteLeadsResponse, BuildError> {
        Ok(NotesDeleteLeadsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
