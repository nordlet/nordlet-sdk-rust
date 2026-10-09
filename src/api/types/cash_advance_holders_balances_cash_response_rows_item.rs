pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdvanceHoldersBalancesCashResponseRowsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub issued: String,
    #[serde(default)]
    pub returned: String,
    #[serde(default)]
    pub settled: String,
    #[serde(default)]
    pub balance: String,
}

impl AdvanceHoldersBalancesCashResponseRowsItem {
    pub fn builder() -> AdvanceHoldersBalancesCashResponseRowsItemBuilder {
        <AdvanceHoldersBalancesCashResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdvanceHoldersBalancesCashResponseRowsItemBuilder {
    employee_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    issued: Option<String>,
    returned: Option<String>,
    settled: Option<String>,
    balance: Option<String>,
}

impl AdvanceHoldersBalancesCashResponseRowsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn issued(mut self, value: impl Into<String>) -> Self {
        self.issued = Some(value.into());
        self
    }

    pub fn returned(mut self, value: impl Into<String>) -> Self {
        self.returned = Some(value.into());
        self
    }

    pub fn settled(mut self, value: impl Into<String>) -> Self {
        self.settled = Some(value.into());
        self
    }

    pub fn balance(mut self, value: impl Into<String>) -> Self {
        self.balance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdvanceHoldersBalancesCashResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](AdvanceHoldersBalancesCashResponseRowsItemBuilder::employee_id)
    /// - [`first_name`](AdvanceHoldersBalancesCashResponseRowsItemBuilder::first_name)
    /// - [`last_name`](AdvanceHoldersBalancesCashResponseRowsItemBuilder::last_name)
    /// - [`issued`](AdvanceHoldersBalancesCashResponseRowsItemBuilder::issued)
    /// - [`returned`](AdvanceHoldersBalancesCashResponseRowsItemBuilder::returned)
    /// - [`settled`](AdvanceHoldersBalancesCashResponseRowsItemBuilder::settled)
    /// - [`balance`](AdvanceHoldersBalancesCashResponseRowsItemBuilder::balance)
    pub fn build(self) -> Result<AdvanceHoldersBalancesCashResponseRowsItem, BuildError> {
        Ok(AdvanceHoldersBalancesCashResponseRowsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            issued: self
                .issued
                .ok_or_else(|| BuildError::missing_field("issued"))?,
            returned: self
                .returned
                .ok_or_else(|| BuildError::missing_field("returned"))?,
            settled: self
                .settled
                .ok_or_else(|| BuildError::missing_field("settled"))?,
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
        })
    }
}
