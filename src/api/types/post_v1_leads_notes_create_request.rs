pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsNotesCreateRequest {
    #[serde(rename = "leadId")]
    #[serde(default)]
    pub lead_id: String,
    #[serde(default)]
    pub body: String,
}

impl PostV1LeadsNotesCreateRequest {
    pub fn builder() -> PostV1LeadsNotesCreateRequestBuilder {
        <PostV1LeadsNotesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsNotesCreateRequestBuilder {
    lead_id: Option<String>,
    body: Option<String>,
}

impl PostV1LeadsNotesCreateRequestBuilder {
    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsNotesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lead_id`](PostV1LeadsNotesCreateRequestBuilder::lead_id)
    /// - [`body`](PostV1LeadsNotesCreateRequestBuilder::body)
    pub fn build(self) -> Result<PostV1LeadsNotesCreateRequest, BuildError> {
        Ok(PostV1LeadsNotesCreateRequest {
            lead_id: self
                .lead_id
                .ok_or_else(|| BuildError::missing_field("lead_id"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
