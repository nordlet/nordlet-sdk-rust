pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingPortalCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1BillingPortalCreateRequestLocale>,
}

impl PostV1BillingPortalCreateRequest {
    pub fn builder() -> PostV1BillingPortalCreateRequestBuilder {
        <PostV1BillingPortalCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingPortalCreateRequestBuilder {
    locale: Option<PostV1BillingPortalCreateRequestLocale>,
}

impl PostV1BillingPortalCreateRequestBuilder {
    pub fn locale(mut self, value: PostV1BillingPortalCreateRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingPortalCreateRequest`].
    pub fn build(self) -> Result<PostV1BillingPortalCreateRequest, BuildError> {
        Ok(PostV1BillingPortalCreateRequest {
            locale: self.locale,
        })
    }
}
