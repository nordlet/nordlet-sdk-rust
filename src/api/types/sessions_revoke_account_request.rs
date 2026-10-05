pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsRevokeAccountRequest {
    #[serde(default)]
    pub id: String,
}

impl SessionsRevokeAccountRequest {
    pub fn builder() -> SessionsRevokeAccountRequestBuilder {
        <SessionsRevokeAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsRevokeAccountRequestBuilder {
    id: Option<String>,
}

impl SessionsRevokeAccountRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SessionsRevokeAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SessionsRevokeAccountRequestBuilder::id)
    pub fn build(self) -> Result<SessionsRevokeAccountRequest, BuildError> {
        Ok(SessionsRevokeAccountRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
