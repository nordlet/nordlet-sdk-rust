pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersRemoveAccountRequest {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}

impl MembersRemoveAccountRequest {
    pub fn builder() -> MembersRemoveAccountRequestBuilder {
        <MembersRemoveAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersRemoveAccountRequestBuilder {
    user_id: Option<String>,
}

impl MembersRemoveAccountRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembersRemoveAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](MembersRemoveAccountRequestBuilder::user_id)
    pub fn build(self) -> Result<MembersRemoveAccountRequest, BuildError> {
        Ok(MembersRemoveAccountRequest {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
