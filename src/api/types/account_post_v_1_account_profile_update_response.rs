pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountProfileUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostV1AccountProfileUpdateResponse {
    pub fn builder() -> PostV1AccountProfileUpdateResponseBuilder {
        <PostV1AccountProfileUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountProfileUpdateResponseBuilder {
    id: Option<String>,
    email: Option<String>,
    name: Option<String>,
}

impl PostV1AccountProfileUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountProfileUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountProfileUpdateResponseBuilder::id)
    /// - [`email`](PostV1AccountProfileUpdateResponseBuilder::email)
    pub fn build(self) -> Result<PostV1AccountProfileUpdateResponse, BuildError> {
        Ok(PostV1AccountProfileUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
        })
    }
}
