pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountSetAccountTypeBillingRequest {
    #[serde(rename = "accountType")]
    pub account_type: AccountSetAccountTypeBillingRequestAccountType,
}

impl AccountSetAccountTypeBillingRequest {
    pub fn builder() -> AccountSetAccountTypeBillingRequestBuilder {
        <AccountSetAccountTypeBillingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSetAccountTypeBillingRequestBuilder {
    account_type: Option<AccountSetAccountTypeBillingRequestAccountType>,
}

impl AccountSetAccountTypeBillingRequestBuilder {
    pub fn account_type(mut self, value: AccountSetAccountTypeBillingRequestAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountSetAccountTypeBillingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](AccountSetAccountTypeBillingRequestBuilder::account_type)
    pub fn build(self) -> Result<AccountSetAccountTypeBillingRequest, BuildError> {
        Ok(AccountSetAccountTypeBillingRequest {
            account_type: self
                .account_type
                .ok_or_else(|| BuildError::missing_field("account_type"))?,
        })
    }
}
