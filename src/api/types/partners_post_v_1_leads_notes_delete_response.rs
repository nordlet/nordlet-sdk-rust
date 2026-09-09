pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsNotesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1LeadsNotesDeleteResponse {
    pub fn builder() -> PostV1LeadsNotesDeleteResponseBuilder {
        <PostV1LeadsNotesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsNotesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1LeadsNotesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsNotesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsNotesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1LeadsNotesDeleteResponse, BuildError> {
        Ok(PostV1LeadsNotesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
