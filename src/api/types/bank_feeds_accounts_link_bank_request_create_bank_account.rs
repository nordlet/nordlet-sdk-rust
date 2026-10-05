pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsAccountsLinkBankRequestCreateBankAccount {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
}

impl FeedsAccountsLinkBankRequestCreateBankAccount {
    pub fn builder() -> FeedsAccountsLinkBankRequestCreateBankAccountBuilder {
        <FeedsAccountsLinkBankRequestCreateBankAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsAccountsLinkBankRequestCreateBankAccountBuilder {
    name: Option<String>,
    account_code: Option<String>,
}

impl FeedsAccountsLinkBankRequestCreateBankAccountBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedsAccountsLinkBankRequestCreateBankAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](FeedsAccountsLinkBankRequestCreateBankAccountBuilder::name)
    pub fn build(self) -> Result<FeedsAccountsLinkBankRequestCreateBankAccount, BuildError> {
        Ok(FeedsAccountsLinkBankRequestCreateBankAccount {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            account_code: self.account_code,
        })
    }
}
