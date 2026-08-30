pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountEmailChangeRequestResponse {
    #[serde(default)]
    pub sent: bool,
}

impl PostV1AccountEmailChangeRequestResponse {
    pub fn builder() -> PostV1AccountEmailChangeRequestResponseBuilder {
        <PostV1AccountEmailChangeRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountEmailChangeRequestResponseBuilder {
    sent: Option<bool>,
}

impl PostV1AccountEmailChangeRequestResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountEmailChangeRequestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](PostV1AccountEmailChangeRequestResponseBuilder::sent)
    pub fn build(self) -> Result<PostV1AccountEmailChangeRequestResponse, BuildError> {
        Ok(PostV1AccountEmailChangeRequestResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
        })
    }
}
