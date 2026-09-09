pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsNotesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1LeadsNotesDeleteRequest {
    pub fn builder() -> PostV1LeadsNotesDeleteRequestBuilder {
        <PostV1LeadsNotesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsNotesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1LeadsNotesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsNotesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsNotesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LeadsNotesDeleteRequest, BuildError> {
        Ok(PostV1LeadsNotesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
