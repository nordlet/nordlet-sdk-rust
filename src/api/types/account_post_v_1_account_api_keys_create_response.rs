pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountApiKeysCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub key: String,
}

impl PostV1AccountApiKeysCreateResponse {
    pub fn builder() -> PostV1AccountApiKeysCreateResponseBuilder {
        <PostV1AccountApiKeysCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountApiKeysCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    scopes: Option<Vec<String>>,
    key: Option<String>,
}

impl PostV1AccountApiKeysCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountApiKeysCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountApiKeysCreateResponseBuilder::id)
    /// - [`name`](PostV1AccountApiKeysCreateResponseBuilder::name)
    /// - [`scopes`](PostV1AccountApiKeysCreateResponseBuilder::scopes)
    /// - [`key`](PostV1AccountApiKeysCreateResponseBuilder::key)
    pub fn build(self) -> Result<PostV1AccountApiKeysCreateResponse, BuildError> {
        Ok(PostV1AccountApiKeysCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
