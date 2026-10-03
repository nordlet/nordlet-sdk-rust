pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountApiKeysRotateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub key: String,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "replacedKeyId")]
    #[serde(default)]
    pub replaced_key_id: String,
    #[serde(rename = "replacedKeyExpiresAt")]
    #[serde(default)]
    pub replaced_key_expires_at: String,
}

impl PostV1AccountApiKeysRotateResponse {
    pub fn builder() -> PostV1AccountApiKeysRotateResponseBuilder {
        <PostV1AccountApiKeysRotateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountApiKeysRotateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    scopes: Option<Vec<String>>,
    key: Option<String>,
    expires_at: Option<String>,
    replaced_key_id: Option<String>,
    replaced_key_expires_at: Option<String>,
}

impl PostV1AccountApiKeysRotateResponseBuilder {
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

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn replaced_key_id(mut self, value: impl Into<String>) -> Self {
        self.replaced_key_id = Some(value.into());
        self
    }

    pub fn replaced_key_expires_at(mut self, value: impl Into<String>) -> Self {
        self.replaced_key_expires_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountApiKeysRotateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountApiKeysRotateResponseBuilder::id)
    /// - [`name`](PostV1AccountApiKeysRotateResponseBuilder::name)
    /// - [`scopes`](PostV1AccountApiKeysRotateResponseBuilder::scopes)
    /// - [`key`](PostV1AccountApiKeysRotateResponseBuilder::key)
    /// - [`replaced_key_id`](PostV1AccountApiKeysRotateResponseBuilder::replaced_key_id)
    /// - [`replaced_key_expires_at`](PostV1AccountApiKeysRotateResponseBuilder::replaced_key_expires_at)
    pub fn build(self) -> Result<PostV1AccountApiKeysRotateResponse, BuildError> {
        Ok(PostV1AccountApiKeysRotateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            expires_at: self.expires_at,
            replaced_key_id: self
                .replaced_key_id
                .ok_or_else(|| BuildError::missing_field("replaced_key_id"))?,
            replaced_key_expires_at: self
                .replaced_key_expires_at
                .ok_or_else(|| BuildError::missing_field("replaced_key_expires_at"))?,
        })
    }
}
