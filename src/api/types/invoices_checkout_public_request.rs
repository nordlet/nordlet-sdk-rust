pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCheckoutPublicRequest {
    #[serde(default)]
    pub token: String,
}

impl InvoicesCheckoutPublicRequest {
    pub fn builder() -> InvoicesCheckoutPublicRequestBuilder {
        <InvoicesCheckoutPublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCheckoutPublicRequestBuilder {
    token: Option<String>,
}

impl InvoicesCheckoutPublicRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCheckoutPublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](InvoicesCheckoutPublicRequestBuilder::token)
    pub fn build(self) -> Result<InvoicesCheckoutPublicRequest, BuildError> {
        Ok(InvoicesCheckoutPublicRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
