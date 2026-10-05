pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankAccountsDeletePartnersResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl BankAccountsDeletePartnersResponse {
    pub fn builder() -> BankAccountsDeletePartnersResponseBuilder {
        <BankAccountsDeletePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountsDeletePartnersResponseBuilder {
    deleted: Option<bool>,
}

impl BankAccountsDeletePartnersResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankAccountsDeletePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](BankAccountsDeletePartnersResponseBuilder::deleted)
    pub fn build(self) -> Result<BankAccountsDeletePartnersResponse, BuildError> {
        Ok(BankAccountsDeletePartnersResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
