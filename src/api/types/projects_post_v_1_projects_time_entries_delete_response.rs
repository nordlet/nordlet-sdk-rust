pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1ProjectsTimeEntriesDeleteResponse {
    pub fn builder() -> PostV1ProjectsTimeEntriesDeleteResponseBuilder {
        <PostV1ProjectsTimeEntriesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1ProjectsTimeEntriesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProjectsTimeEntriesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesDeleteResponse, BuildError> {
        Ok(PostV1ProjectsTimeEntriesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
