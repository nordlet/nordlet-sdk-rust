pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersTransferOwnershipAccountRequest {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(rename = "movePayer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_payer: Option<bool>,
}

impl MembersTransferOwnershipAccountRequest {
    pub fn builder() -> MembersTransferOwnershipAccountRequestBuilder {
        <MembersTransferOwnershipAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersTransferOwnershipAccountRequestBuilder {
    user_id: Option<String>,
    move_payer: Option<bool>,
}

impl MembersTransferOwnershipAccountRequestBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn move_payer(mut self, value: bool) -> Self {
        self.move_payer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembersTransferOwnershipAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](MembersTransferOwnershipAccountRequestBuilder::user_id)
    pub fn build(self) -> Result<MembersTransferOwnershipAccountRequest, BuildError> {
        Ok(MembersTransferOwnershipAccountRequest {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            move_payer: self.move_payer,
        })
    }
}
