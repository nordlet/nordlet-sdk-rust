pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountEmailChangeRequestRequest {
    #[serde(rename = "newEmail")]
    #[serde(default)]
    pub new_email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1AccountEmailChangeRequestRequestLocale>,
}

impl PostV1AccountEmailChangeRequestRequest {
    pub fn builder() -> PostV1AccountEmailChangeRequestRequestBuilder {
        <PostV1AccountEmailChangeRequestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountEmailChangeRequestRequestBuilder {
    new_email: Option<String>,
    locale: Option<PostV1AccountEmailChangeRequestRequestLocale>,
}

impl PostV1AccountEmailChangeRequestRequestBuilder {
    pub fn new_email(mut self, value: impl Into<String>) -> Self {
        self.new_email = Some(value.into());
        self
    }

    pub fn locale(mut self, value: PostV1AccountEmailChangeRequestRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountEmailChangeRequestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`new_email`](PostV1AccountEmailChangeRequestRequestBuilder::new_email)
    pub fn build(self) -> Result<PostV1AccountEmailChangeRequestRequest, BuildError> {
        Ok(PostV1AccountEmailChangeRequestRequest {
            new_email: self
                .new_email
                .ok_or_else(|| BuildError::missing_field("new_email"))?,
            locale: self.locale,
        })
    }
}
