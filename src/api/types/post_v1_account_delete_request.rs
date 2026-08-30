pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountDeleteRequest {
    #[serde(rename = "confirmEmail")]
    #[serde(default)]
    pub confirm_email: String,
}

impl PostV1AccountDeleteRequest {
    pub fn builder() -> PostV1AccountDeleteRequestBuilder {
        <PostV1AccountDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountDeleteRequestBuilder {
    confirm_email: Option<String>,
}

impl PostV1AccountDeleteRequestBuilder {
    pub fn confirm_email(mut self, value: impl Into<String>) -> Self {
        self.confirm_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`confirm_email`](PostV1AccountDeleteRequestBuilder::confirm_email)
    pub fn build(self) -> Result<PostV1AccountDeleteRequest, BuildError> {
        Ok(PostV1AccountDeleteRequest {
            confirm_email: self
                .confirm_email
                .ok_or_else(|| BuildError::missing_field("confirm_email"))?,
        })
    }
}
