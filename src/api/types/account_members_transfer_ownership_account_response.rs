pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersTransferOwnershipAccountResponse {
    #[serde(rename = "ownerUserId")]
    #[serde(default)]
    pub owner_user_id: String,
    #[serde(rename = "previousOwnerRole")]
    #[serde(default)]
    pub previous_owner_role: String,
    #[serde(rename = "payerUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_user_id: Option<String>,
}

impl MembersTransferOwnershipAccountResponse {
    pub fn builder() -> MembersTransferOwnershipAccountResponseBuilder {
        <MembersTransferOwnershipAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersTransferOwnershipAccountResponseBuilder {
    owner_user_id: Option<String>,
    previous_owner_role: Option<String>,
    payer_user_id: Option<String>,
}

impl MembersTransferOwnershipAccountResponseBuilder {
    pub fn owner_user_id(mut self, value: impl Into<String>) -> Self {
        self.owner_user_id = Some(value.into());
        self
    }

    pub fn previous_owner_role(mut self, value: impl Into<String>) -> Self {
        self.previous_owner_role = Some(value.into());
        self
    }

    pub fn payer_user_id(mut self, value: impl Into<String>) -> Self {
        self.payer_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembersTransferOwnershipAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`owner_user_id`](MembersTransferOwnershipAccountResponseBuilder::owner_user_id)
    /// - [`previous_owner_role`](MembersTransferOwnershipAccountResponseBuilder::previous_owner_role)
    pub fn build(self) -> Result<MembersTransferOwnershipAccountResponse, BuildError> {
        Ok(MembersTransferOwnershipAccountResponse {
            owner_user_id: self
                .owner_user_id
                .ok_or_else(|| BuildError::missing_field("owner_user_id"))?,
            previous_owner_role: self
                .previous_owner_role
                .ok_or_else(|| BuildError::missing_field("previous_owner_role"))?,
            payer_user_id: self.payer_user_id,
        })
    }
}
