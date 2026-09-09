pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1LeadsDeleteResponse {
    pub fn builder() -> PostV1LeadsDeleteResponseBuilder {
        <PostV1LeadsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1LeadsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1LeadsDeleteResponse, BuildError> {
        Ok(PostV1LeadsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
