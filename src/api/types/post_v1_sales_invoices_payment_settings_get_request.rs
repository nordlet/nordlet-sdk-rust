pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPaymentSettingsGetRequest {}

impl PostV1SalesInvoicesPaymentSettingsGetRequest {
    pub fn builder() -> PostV1SalesInvoicesPaymentSettingsGetRequestBuilder {
        <PostV1SalesInvoicesPaymentSettingsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPaymentSettingsGetRequestBuilder {}

impl PostV1SalesInvoicesPaymentSettingsGetRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPaymentSettingsGetRequest`].
    pub fn build(self) -> Result<PostV1SalesInvoicesPaymentSettingsGetRequest, BuildError> {
        Ok(PostV1SalesInvoicesPaymentSettingsGetRequest {})
    }
}
