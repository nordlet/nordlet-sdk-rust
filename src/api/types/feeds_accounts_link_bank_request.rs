pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsAccountsLinkBankRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "bankAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_id: Option<String>,
    #[serde(rename = "createBankAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_bank_account: Option<FeedsAccountsLinkBankRequestCreateBankAccount>,
    #[serde(rename = "syncFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_from: Option<NaiveDate>,
}

impl FeedsAccountsLinkBankRequest {
    pub fn builder() -> FeedsAccountsLinkBankRequestBuilder {
        <FeedsAccountsLinkBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsAccountsLinkBankRequestBuilder {
    id: Option<String>,
    bank_account_id: Option<String>,
    create_bank_account: Option<FeedsAccountsLinkBankRequestCreateBankAccount>,
    sync_from: Option<NaiveDate>,
}

impl FeedsAccountsLinkBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn create_bank_account(
        mut self,
        value: FeedsAccountsLinkBankRequestCreateBankAccount,
    ) -> Self {
        self.create_bank_account = Some(value);
        self
    }

    pub fn sync_from(mut self, value: NaiveDate) -> Self {
        self.sync_from = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsAccountsLinkBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FeedsAccountsLinkBankRequestBuilder::id)
    pub fn build(self) -> Result<FeedsAccountsLinkBankRequest, BuildError> {
        Ok(FeedsAccountsLinkBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            bank_account_id: self.bank_account_id,
            create_bank_account: self.create_bank_account,
            sync_from: self.sync_from,
        })
    }
}
