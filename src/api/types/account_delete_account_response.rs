pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAccountResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl DeleteAccountResponse {
    pub fn builder() -> DeleteAccountResponseBuilder {
        <DeleteAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAccountResponseBuilder {
    deleted: Option<bool>,
}

impl DeleteAccountResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteAccountResponseBuilder::deleted)
    pub fn build(self) -> Result<DeleteAccountResponse, BuildError> {
        Ok(DeleteAccountResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
