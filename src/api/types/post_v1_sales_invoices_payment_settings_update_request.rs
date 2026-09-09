pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPaymentSettingsUpdateRequest {
    #[serde(rename = "paymentLinkTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link_template: Option<String>,
}

impl PostV1SalesInvoicesPaymentSettingsUpdateRequest {
    pub fn builder() -> PostV1SalesInvoicesPaymentSettingsUpdateRequestBuilder {
        <PostV1SalesInvoicesPaymentSettingsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPaymentSettingsUpdateRequestBuilder {
    payment_link_template: Option<String>,
}

impl PostV1SalesInvoicesPaymentSettingsUpdateRequestBuilder {
    pub fn payment_link_template(mut self, value: impl Into<String>) -> Self {
        self.payment_link_template = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPaymentSettingsUpdateRequest`].
    pub fn build(self) -> Result<PostV1SalesInvoicesPaymentSettingsUpdateRequest, BuildError> {
        Ok(PostV1SalesInvoicesPaymentSettingsUpdateRequest {
            payment_link_template: self.payment_link_template,
        })
    }
}
