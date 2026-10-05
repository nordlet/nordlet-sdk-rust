pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PortalCreateBillingResponse {
    #[serde(default)]
    pub url: String,
}

impl PortalCreateBillingResponse {
    pub fn builder() -> PortalCreateBillingResponseBuilder {
        <PortalCreateBillingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PortalCreateBillingResponseBuilder {
    url: Option<String>,
}

impl PortalCreateBillingResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PortalCreateBillingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](PortalCreateBillingResponseBuilder::url)
    pub fn build(self) -> Result<PortalCreateBillingResponse, BuildError> {
        Ok(PortalCreateBillingResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
