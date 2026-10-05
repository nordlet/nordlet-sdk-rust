pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankAccountsDeletePartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl BankAccountsDeletePartnersRequest {
    pub fn builder() -> BankAccountsDeletePartnersRequestBuilder {
        <BankAccountsDeletePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountsDeletePartnersRequestBuilder {
    id: Option<String>,
}

impl BankAccountsDeletePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BankAccountsDeletePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BankAccountsDeletePartnersRequestBuilder::id)
    pub fn build(self) -> Result<BankAccountsDeletePartnersRequest, BuildError> {
        Ok(BankAccountsDeletePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
