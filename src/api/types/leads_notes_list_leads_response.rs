pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotesListLeadsResponse {
    #[serde(default)]
    pub rows: Vec<NotesListLeadsResponseRowsItem>,
}

impl NotesListLeadsResponse {
    pub fn builder() -> NotesListLeadsResponseBuilder {
        <NotesListLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotesListLeadsResponseBuilder {
    rows: Option<Vec<NotesListLeadsResponseRowsItem>>,
}

impl NotesListLeadsResponseBuilder {
    pub fn rows(mut self, value: Vec<NotesListLeadsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotesListLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](NotesListLeadsResponseBuilder::rows)
    pub fn build(self) -> Result<NotesListLeadsResponse, BuildError> {
        Ok(NotesListLeadsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
