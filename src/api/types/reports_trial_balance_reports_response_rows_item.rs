pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TrialBalanceReportsResponseRowsItem {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub r#type: TrialBalanceReportsResponseRowsItemType,
    #[serde(default)]
    pub opening: String,
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
    #[serde(default)]
    pub closing: String,
}

impl TrialBalanceReportsResponseRowsItem {
    pub fn builder() -> TrialBalanceReportsResponseRowsItemBuilder {
        <TrialBalanceReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrialBalanceReportsResponseRowsItemBuilder {
    account_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    r#type: Option<TrialBalanceReportsResponseRowsItemType>,
    opening: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
    closing: Option<String>,
}

impl TrialBalanceReportsResponseRowsItemBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: TrialBalanceReportsResponseRowsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn opening(mut self, value: impl Into<String>) -> Self {
        self.opening = Some(value.into());
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

    pub fn closing(mut self, value: impl Into<String>) -> Self {
        self.closing = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TrialBalanceReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](TrialBalanceReportsResponseRowsItemBuilder::account_id)
    /// - [`code`](TrialBalanceReportsResponseRowsItemBuilder::code)
    /// - [`name`](TrialBalanceReportsResponseRowsItemBuilder::name)
    /// - [`r#type`](TrialBalanceReportsResponseRowsItemBuilder::r#type)
    /// - [`opening`](TrialBalanceReportsResponseRowsItemBuilder::opening)
    /// - [`debit`](TrialBalanceReportsResponseRowsItemBuilder::debit)
    /// - [`credit`](TrialBalanceReportsResponseRowsItemBuilder::credit)
    /// - [`closing`](TrialBalanceReportsResponseRowsItemBuilder::closing)
    pub fn build(self) -> Result<TrialBalanceReportsResponseRowsItem, BuildError> {
        Ok(TrialBalanceReportsResponseRowsItem {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            opening: self
                .opening
                .ok_or_else(|| BuildError::missing_field("opening"))?,
            debit: self
                .debit
                .ok_or_else(|| BuildError::missing_field("debit"))?,
            credit: self
                .credit
                .ok_or_else(|| BuildError::missing_field("credit"))?,
            closing: self
                .closing
                .ok_or_else(|| BuildError::missing_field("closing"))?,
        })
    }
}
