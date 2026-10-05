pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PortalCreateBillingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PortalCreateBillingRequestLocale>,
}

impl PortalCreateBillingRequest {
    pub fn builder() -> PortalCreateBillingRequestBuilder {
        <PortalCreateBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PortalCreateBillingRequestBuilder {
    locale: Option<PortalCreateBillingRequestLocale>,
}

impl PortalCreateBillingRequestBuilder {
    pub fn locale(mut self, value: PortalCreateBillingRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PortalCreateBillingRequest`].
    pub fn build(self) -> Result<PortalCreateBillingRequest, BuildError> {
        Ok(PortalCreateBillingRequest {
            locale: self.locale,
        })
    }
}
