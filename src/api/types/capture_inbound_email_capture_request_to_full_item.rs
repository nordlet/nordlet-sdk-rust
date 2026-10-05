pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InboundEmailCaptureRequestToFullItem {
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl InboundEmailCaptureRequestToFullItem {
    pub fn builder() -> InboundEmailCaptureRequestToFullItemBuilder {
        <InboundEmailCaptureRequestToFullItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboundEmailCaptureRequestToFullItemBuilder {
    email: Option<String>,
}

impl InboundEmailCaptureRequestToFullItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InboundEmailCaptureRequestToFullItem`].
    pub fn build(self) -> Result<InboundEmailCaptureRequestToFullItem, BuildError> {
        Ok(InboundEmailCaptureRequestToFullItem {
            email: self.email,
            extra: Default::default(),
        })
    }
}
