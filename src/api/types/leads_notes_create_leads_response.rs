pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotesCreateLeadsResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "leadId")]
    #[serde(default)]
    pub lead_id: String,
    #[serde(default)]
    pub body: String,
    #[serde(rename = "authorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl NotesCreateLeadsResponse {
    pub fn builder() -> NotesCreateLeadsResponseBuilder {
        <NotesCreateLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotesCreateLeadsResponseBuilder {
    id: Option<String>,
    lead_id: Option<String>,
    body: Option<String>,
    author_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl NotesCreateLeadsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn author_id(mut self, value: impl Into<String>) -> Self {
        self.author_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotesCreateLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](NotesCreateLeadsResponseBuilder::id)
    /// - [`lead_id`](NotesCreateLeadsResponseBuilder::lead_id)
    /// - [`body`](NotesCreateLeadsResponseBuilder::body)
    /// - [`created_at`](NotesCreateLeadsResponseBuilder::created_at)
    pub fn build(self) -> Result<NotesCreateLeadsResponse, BuildError> {
        Ok(NotesCreateLeadsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lead_id: self
                .lead_id
                .ok_or_else(|| BuildError::missing_field("lead_id"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            author_id: self.author_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
