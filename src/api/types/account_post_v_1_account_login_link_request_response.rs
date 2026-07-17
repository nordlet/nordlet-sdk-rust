pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountLoginLinkRequestResponse {
    #[serde(default)]
    pub sent: bool,
}

impl PostV1AccountLoginLinkRequestResponse {
    pub fn builder() -> PostV1AccountLoginLinkRequestResponseBuilder {
        <PostV1AccountLoginLinkRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountLoginLinkRequestResponseBuilder {
    sent: Option<bool>,
}

impl PostV1AccountLoginLinkRequestResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountLoginLinkRequestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](PostV1AccountLoginLinkRequestResponseBuilder::sent)
    pub fn build(self) -> Result<PostV1AccountLoginLinkRequestResponse, BuildError> {
        Ok(PostV1AccountLoginLinkRequestResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
        })
    }
}
