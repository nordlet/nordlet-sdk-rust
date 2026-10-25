pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesGetPublicRequest {
    #[serde(default)]
    pub token: String,
}

impl InvoicesGetPublicRequest {
    pub fn builder() -> InvoicesGetPublicRequestBuilder {
        <InvoicesGetPublicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesGetPublicRequestBuilder {
    token: Option<String>,
}

impl InvoicesGetPublicRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesGetPublicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](InvoicesGetPublicRequestBuilder::token)
    pub fn build(self) -> Result<InvoicesGetPublicRequest, BuildError> {
        Ok(InvoicesGetPublicRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
