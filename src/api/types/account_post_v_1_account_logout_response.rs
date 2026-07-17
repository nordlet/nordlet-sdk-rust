pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountLogoutResponse {
    #[serde(rename = "loggedOut")]
    #[serde(default)]
    pub logged_out: bool,
}

impl PostV1AccountLogoutResponse {
    pub fn builder() -> PostV1AccountLogoutResponseBuilder {
        <PostV1AccountLogoutResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountLogoutResponseBuilder {
    logged_out: Option<bool>,
}

impl PostV1AccountLogoutResponseBuilder {
    pub fn logged_out(mut self, value: bool) -> Self {
        self.logged_out = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountLogoutResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`logged_out`](PostV1AccountLogoutResponseBuilder::logged_out)
    pub fn build(self) -> Result<PostV1AccountLogoutResponse, BuildError> {
        Ok(PostV1AccountLogoutResponse {
            logged_out: self
                .logged_out
                .ok_or_else(|| BuildError::missing_field("logged_out"))?,
        })
    }
}
