pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsAccountsLinkRequestCreateBankAccount {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
}

impl PostV1BankFeedsAccountsLinkRequestCreateBankAccount {
    pub fn builder() -> PostV1BankFeedsAccountsLinkRequestCreateBankAccountBuilder {
        <PostV1BankFeedsAccountsLinkRequestCreateBankAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsAccountsLinkRequestCreateBankAccountBuilder {
    name: Option<String>,
    account_code: Option<String>,
}

impl PostV1BankFeedsAccountsLinkRequestCreateBankAccountBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsAccountsLinkRequestCreateBankAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1BankFeedsAccountsLinkRequestCreateBankAccountBuilder::name)
    pub fn build(self) -> Result<PostV1BankFeedsAccountsLinkRequestCreateBankAccount, BuildError> {
        Ok(PostV1BankFeedsAccountsLinkRequestCreateBankAccount {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            account_code: self.account_code,
        })
    }
}
