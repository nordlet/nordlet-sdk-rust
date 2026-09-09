pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPaymentSettingsGetResponse {
    #[serde(rename = "paymentLinkTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link_template: Option<String>,
}

impl PostV1SalesInvoicesPaymentSettingsGetResponse {
    pub fn builder() -> PostV1SalesInvoicesPaymentSettingsGetResponseBuilder {
        <PostV1SalesInvoicesPaymentSettingsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPaymentSettingsGetResponseBuilder {
    payment_link_template: Option<String>,
}

impl PostV1SalesInvoicesPaymentSettingsGetResponseBuilder {
    pub fn payment_link_template(mut self, value: impl Into<String>) -> Self {
        self.payment_link_template = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPaymentSettingsGetResponse`].
    pub fn build(self) -> Result<PostV1SalesInvoicesPaymentSettingsGetResponse, BuildError> {
        Ok(PostV1SalesInvoicesPaymentSettingsGetResponse {
            payment_link_template: self.payment_link_template,
        })
    }
}
