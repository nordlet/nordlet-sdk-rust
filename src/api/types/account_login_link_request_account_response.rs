pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoginLinkRequestAccountResponse {
    #[serde(default)]
    pub sent: bool,
}

impl LoginLinkRequestAccountResponse {
    pub fn builder() -> LoginLinkRequestAccountResponseBuilder {
        <LoginLinkRequestAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoginLinkRequestAccountResponseBuilder {
    sent: Option<bool>,
}

impl LoginLinkRequestAccountResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LoginLinkRequestAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](LoginLinkRequestAccountResponseBuilder::sent)
    pub fn build(self) -> Result<LoginLinkRequestAccountResponse, BuildError> {
        Ok(LoginLinkRequestAccountResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
        })
    }
}
