pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountProfileUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostV1AccountProfileUpdateRequest {
    pub fn builder() -> PostV1AccountProfileUpdateRequestBuilder {
        <PostV1AccountProfileUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountProfileUpdateRequestBuilder {
    name: Option<String>,
}

impl PostV1AccountProfileUpdateRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountProfileUpdateRequest`].
    pub fn build(self) -> Result<PostV1AccountProfileUpdateRequest, BuildError> {
        Ok(PostV1AccountProfileUpdateRequest { name: self.name })
    }
}
