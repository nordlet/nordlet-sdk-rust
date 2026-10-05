pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterActivityReportsResponseRowsItem {
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "accountName")]
    #[serde(default)]
    pub account_name: String,
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
    #[serde(default)]
    pub net: String,
}

impl CostCenterActivityReportsResponseRowsItem {
    pub fn builder() -> CostCenterActivityReportsResponseRowsItemBuilder {
        <CostCenterActivityReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterActivityReportsResponseRowsItemBuilder {
    account_code: Option<String>,
    account_name: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
    net: Option<String>,
}

impl CostCenterActivityReportsResponseRowsItemBuilder {
    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn account_name(mut self, value: impl Into<String>) -> Self {
        self.account_name = Some(value.into());
        self
    }

    pub fn debit(mut self, value: impl Into<String>) -> Self {
        self.debit = Some(value.into());
        self
    }

    pub fn credit(mut self, value: impl Into<String>) -> Self {
        self.credit = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CostCenterActivityReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_code`](CostCenterActivityReportsResponseRowsItemBuilder::account_code)
    /// - [`account_name`](CostCenterActivityReportsResponseRowsItemBuilder::account_name)
    /// - [`debit`](CostCenterActivityReportsResponseRowsItemBuilder::debit)
    /// - [`credit`](CostCenterActivityReportsResponseRowsItemBuilder::credit)
    /// - [`net`](CostCenterActivityReportsResponseRowsItemBuilder::net)
    pub fn build(self) -> Result<CostCenterActivityReportsResponseRowsItem, BuildError> {
        Ok(CostCenterActivityReportsResponseRowsItem {
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            account_name: self
                .account_name
                .ok_or_else(|| BuildError::missing_field("account_name"))?,
            debit: self
                .debit
                .ok_or_else(|| BuildError::missing_field("debit"))?,
            credit: self
                .credit
                .ok_or_else(|| BuildError::missing_field("credit"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}
