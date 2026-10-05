pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersRemoveAccountResponse {
    #[serde(default)]
    pub removed: bool,
}

impl MembersRemoveAccountResponse {
    pub fn builder() -> MembersRemoveAccountResponseBuilder {
        <MembersRemoveAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersRemoveAccountResponseBuilder {
    removed: Option<bool>,
}

impl MembersRemoveAccountResponseBuilder {
    pub fn removed(mut self, value: bool) -> Self {
        self.removed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembersRemoveAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`removed`](MembersRemoveAccountResponseBuilder::removed)
    pub fn build(self) -> Result<MembersRemoveAccountResponse, BuildError> {
        Ok(MembersRemoveAccountResponse {
            removed: self
                .removed
                .ok_or_else(|| BuildError::missing_field("removed"))?,
        })
    }
}
