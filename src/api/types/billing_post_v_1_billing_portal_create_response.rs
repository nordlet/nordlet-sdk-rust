pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingPortalCreateResponse {
    #[serde(default)]
    pub url: String,
}

impl PostV1BillingPortalCreateResponse {
    pub fn builder() -> PostV1BillingPortalCreateResponseBuilder {
        <PostV1BillingPortalCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingPortalCreateResponseBuilder {
    url: Option<String>,
}

impl PostV1BillingPortalCreateResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingPortalCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](PostV1BillingPortalCreateResponseBuilder::url)
    pub fn build(self) -> Result<PostV1BillingPortalCreateResponse, BuildError> {
        Ok(PostV1BillingPortalCreateResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
