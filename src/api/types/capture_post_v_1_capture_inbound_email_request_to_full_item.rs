pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1CaptureInboundEmailRequestToFullItem {
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl PostV1CaptureInboundEmailRequestToFullItem {
    pub fn builder() -> PostV1CaptureInboundEmailRequestToFullItemBuilder {
        <PostV1CaptureInboundEmailRequestToFullItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureInboundEmailRequestToFullItemBuilder {
    email: Option<String>,
}

impl PostV1CaptureInboundEmailRequestToFullItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureInboundEmailRequestToFullItem`].
    pub fn build(self) -> Result<PostV1CaptureInboundEmailRequestToFullItem, BuildError> {
        Ok(PostV1CaptureInboundEmailRequestToFullItem {
            email: self.email,
            extra: Default::default(),
        })
    }
}
