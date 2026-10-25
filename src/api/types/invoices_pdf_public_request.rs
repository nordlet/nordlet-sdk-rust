pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPdfPublicRequest {
    #[serde(default)]
    pub token: String,
}

impl InvoicesPdfPublicRequest {
    pub fn builder() -> InvoicesPdfPublicRequestBuilder {
        <InvoicesPdfPublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPdfPublicRequestBuilder {
    token: Option<String>,
}

impl InvoicesPdfPublicRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPdfPublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](InvoicesPdfPublicRequestBuilder::token)
    pub fn build(self) -> Result<InvoicesPdfPublicRequest, BuildError> {
        Ok(InvoicesPdfPublicRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
