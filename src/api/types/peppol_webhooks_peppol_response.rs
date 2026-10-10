pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksPeppolResponse {
    #[serde(default)]
    pub handled: bool,
}

impl WebhooksPeppolResponse {
    pub fn builder() -> WebhooksPeppolResponseBuilder {
        <WebhooksPeppolResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksPeppolResponseBuilder {
    handled: Option<bool>,
}

impl WebhooksPeppolResponseBuilder {
    pub fn handled(mut self, value: bool) -> Self {
        self.handled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksPeppolResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`handled`](WebhooksPeppolResponseBuilder::handled)
    pub fn build(self) -> Result<WebhooksPeppolResponse, BuildError> {
        Ok(WebhooksPeppolResponse {
            handled: self
                .handled
                .ok_or_else(|| BuildError::missing_field("handled"))?,
        })
    }
}
