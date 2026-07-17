pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(rename = "inviteUrl")]
    #[serde(default)]
    pub invite_url: String,
    #[serde(rename = "emailSent")]
    #[serde(default)]
    pub email_sent: bool,
}

impl PostV1AccountInvitesCreateResponse {
    pub fn builder() -> PostV1AccountInvitesCreateResponseBuilder {
        <PostV1AccountInvitesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesCreateResponseBuilder {
    id: Option<String>,
    email: Option<String>,
    role: Option<String>,
    expires_at: Option<String>,
    invite_url: Option<String>,
    email_sent: Option<bool>,
}

impl PostV1AccountInvitesCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn invite_url(mut self, value: impl Into<String>) -> Self {
        self.invite_url = Some(value.into());
        self
    }

    pub fn email_sent(mut self, value: bool) -> Self {
        self.email_sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountInvitesCreateResponseBuilder::id)
    /// - [`email`](PostV1AccountInvitesCreateResponseBuilder::email)
    /// - [`role`](PostV1AccountInvitesCreateResponseBuilder::role)
    /// - [`expires_at`](PostV1AccountInvitesCreateResponseBuilder::expires_at)
    /// - [`invite_url`](PostV1AccountInvitesCreateResponseBuilder::invite_url)
    /// - [`email_sent`](PostV1AccountInvitesCreateResponseBuilder::email_sent)
    pub fn build(self) -> Result<PostV1AccountInvitesCreateResponse, BuildError> {
        Ok(PostV1AccountInvitesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            invite_url: self
                .invite_url
                .ok_or_else(|| BuildError::missing_field("invite_url"))?,
            email_sent: self
                .email_sent
                .ok_or_else(|| BuildError::missing_field("email_sent"))?,
        })
    }
}
