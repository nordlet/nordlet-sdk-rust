pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingTopupCreateResponse {
    #[serde(default)]
    pub url: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

impl PostV1BillingTopupCreateResponse {
    pub fn builder() -> PostV1BillingTopupCreateResponseBuilder {
        <PostV1BillingTopupCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingTopupCreateResponseBuilder {
    url: Option<String>,
    session_id: Option<String>,
}

impl PostV1BillingTopupCreateResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingTopupCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](PostV1BillingTopupCreateResponseBuilder::url)
    /// - [`session_id`](PostV1BillingTopupCreateResponseBuilder::session_id)
    pub fn build(self) -> Result<PostV1BillingTopupCreateResponse, BuildError> {
        Ok(PostV1BillingTopupCreateResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}
