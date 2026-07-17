pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesSendResponse {
    #[serde(default)]
    pub sent: bool,
    #[serde(default)]
    pub to: String,
}

impl PostV1SalesInvoicesSendResponse {
    pub fn builder() -> PostV1SalesInvoicesSendResponseBuilder {
        <PostV1SalesInvoicesSendResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesSendResponseBuilder {
    sent: Option<bool>,
    to: Option<String>,
}

impl PostV1SalesInvoicesSendResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesSendResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](PostV1SalesInvoicesSendResponseBuilder::sent)
    /// - [`to`](PostV1SalesInvoicesSendResponseBuilder::to)
    pub fn build(self) -> Result<PostV1SalesInvoicesSendResponse, BuildError> {
        Ok(PostV1SalesInvoicesSendResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
