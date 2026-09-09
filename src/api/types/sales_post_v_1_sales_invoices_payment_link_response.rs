pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPaymentLinkResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PostV1SalesInvoicesPaymentLinkResponseSource>,
}

impl PostV1SalesInvoicesPaymentLinkResponse {
    pub fn builder() -> PostV1SalesInvoicesPaymentLinkResponseBuilder {
        <PostV1SalesInvoicesPaymentLinkResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPaymentLinkResponseBuilder {
    url: Option<String>,
    source: Option<PostV1SalesInvoicesPaymentLinkResponseSource>,
}

impl PostV1SalesInvoicesPaymentLinkResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn source(mut self, value: PostV1SalesInvoicesPaymentLinkResponseSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPaymentLinkResponse`].
    pub fn build(self) -> Result<PostV1SalesInvoicesPaymentLinkResponse, BuildError> {
        Ok(PostV1SalesInvoicesPaymentLinkResponse {
            url: self.url,
            source: self.source,
        })
    }
}
