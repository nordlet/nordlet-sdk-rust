pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesCreateRequest {
    #[serde(default)]
    pub email: String,
    pub role: PostV1AccountInvitesCreateRequestRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1AccountInvitesCreateRequestLocale>,
}

impl PostV1AccountInvitesCreateRequest {
    pub fn builder() -> PostV1AccountInvitesCreateRequestBuilder {
        <PostV1AccountInvitesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesCreateRequestBuilder {
    email: Option<String>,
    role: Option<PostV1AccountInvitesCreateRequestRole>,
    locale: Option<PostV1AccountInvitesCreateRequestLocale>,
}

impl PostV1AccountInvitesCreateRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: PostV1AccountInvitesCreateRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn locale(mut self, value: PostV1AccountInvitesCreateRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](PostV1AccountInvitesCreateRequestBuilder::email)
    /// - [`role`](PostV1AccountInvitesCreateRequestBuilder::role)
    pub fn build(self) -> Result<PostV1AccountInvitesCreateRequest, BuildError> {
        Ok(PostV1AccountInvitesCreateRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            locale: self.locale,
        })
    }
}
