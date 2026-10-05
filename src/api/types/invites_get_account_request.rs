pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesGetAccountRequest {
    #[serde(default)]
    pub token: String,
}

impl InvitesGetAccountRequest {
    pub fn builder() -> InvitesGetAccountRequestBuilder {
        <InvitesGetAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesGetAccountRequestBuilder {
    token: Option<String>,
}

impl InvitesGetAccountRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvitesGetAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](InvitesGetAccountRequestBuilder::token)
    pub fn build(self) -> Result<InvitesGetAccountRequest, BuildError> {
        Ok(InvitesGetAccountRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
