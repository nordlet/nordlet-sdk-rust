pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCheckoutPublicResponse {
    #[serde(default)]
    pub url: String,
}

impl InvoicesCheckoutPublicResponse {
    pub fn builder() -> InvoicesCheckoutPublicResponseBuilder {
        <InvoicesCheckoutPublicResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCheckoutPublicResponseBuilder {
    url: Option<String>,
}

impl InvoicesCheckoutPublicResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCheckoutPublicResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](InvoicesCheckoutPublicResponseBuilder::url)
    pub fn build(self) -> Result<InvoicesCheckoutPublicResponse, BuildError> {
        Ok(InvoicesCheckoutPublicResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
