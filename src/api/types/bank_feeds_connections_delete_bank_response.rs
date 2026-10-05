pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsConnectionsDeleteBankResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl FeedsConnectionsDeleteBankResponse {
    pub fn builder() -> FeedsConnectionsDeleteBankResponseBuilder {
        <FeedsConnectionsDeleteBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsConnectionsDeleteBankResponseBuilder {
    deleted: Option<bool>,
}

impl FeedsConnectionsDeleteBankResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsConnectionsDeleteBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](FeedsConnectionsDeleteBankResponseBuilder::deleted)
    pub fn build(self) -> Result<FeedsConnectionsDeleteBankResponse, BuildError> {
        Ok(FeedsConnectionsDeleteBankResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
