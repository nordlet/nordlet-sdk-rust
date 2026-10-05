pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesRevokeAccountRequest {
    #[serde(default)]
    pub id: String,
}

impl InvitesRevokeAccountRequest {
    pub fn builder() -> InvitesRevokeAccountRequestBuilder {
        <InvitesRevokeAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesRevokeAccountRequestBuilder {
    id: Option<String>,
}

impl InvitesRevokeAccountRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvitesRevokeAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvitesRevokeAccountRequestBuilder::id)
    pub fn build(self) -> Result<InvitesRevokeAccountRequest, BuildError> {
        Ok(InvitesRevokeAccountRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
