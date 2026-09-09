pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesPaymentSettingsUpdateResponse {
    #[serde(rename = "paymentLinkTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link_template: Option<String>,
}

impl PostV1SalesInvoicesPaymentSettingsUpdateResponse {
    pub fn builder() -> PostV1SalesInvoicesPaymentSettingsUpdateResponseBuilder {
        <PostV1SalesInvoicesPaymentSettingsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesPaymentSettingsUpdateResponseBuilder {
    payment_link_template: Option<String>,
}

impl PostV1SalesInvoicesPaymentSettingsUpdateResponseBuilder {
    pub fn payment_link_template(mut self, value: impl Into<String>) -> Self {
        self.payment_link_template = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesPaymentSettingsUpdateResponse`].
    pub fn build(self) -> Result<PostV1SalesInvoicesPaymentSettingsUpdateResponse, BuildError> {
        Ok(PostV1SalesInvoicesPaymentSettingsUpdateResponse {
            payment_link_template: self.payment_link_template,
        })
    }
}
