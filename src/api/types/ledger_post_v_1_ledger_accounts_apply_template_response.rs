pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerAccountsApplyTemplateResponse {
    #[serde(default)]
    pub accounts: i64,
}

impl PostV1LedgerAccountsApplyTemplateResponse {
    pub fn builder() -> PostV1LedgerAccountsApplyTemplateResponseBuilder {
        <PostV1LedgerAccountsApplyTemplateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerAccountsApplyTemplateResponseBuilder {
    accounts: Option<i64>,
}

impl PostV1LedgerAccountsApplyTemplateResponseBuilder {
    pub fn accounts(mut self, value: i64) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerAccountsApplyTemplateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accounts`](PostV1LedgerAccountsApplyTemplateResponseBuilder::accounts)
    pub fn build(self) -> Result<PostV1LedgerAccountsApplyTemplateResponse, BuildError> {
        Ok(PostV1LedgerAccountsApplyTemplateResponse {
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
