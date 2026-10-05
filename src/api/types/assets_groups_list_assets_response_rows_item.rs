pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsListAssetsResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "defaultUsefulLifeMonths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_useful_life_months: Option<i64>,
    #[serde(rename = "assetAccountCode")]
    #[serde(default)]
    pub asset_account_code: String,
    #[serde(rename = "depreciationAccountCode")]
    #[serde(default)]
    pub depreciation_account_code: String,
    #[serde(rename = "expenseAccountCode")]
    #[serde(default)]
    pub expense_account_code: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl GroupsListAssetsResponseRowsItem {
    pub fn builder() -> GroupsListAssetsResponseRowsItemBuilder {
        <GroupsListAssetsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsListAssetsResponseRowsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    default_useful_life_months: Option<i64>,
    asset_account_code: Option<String>,
    depreciation_account_code: Option<String>,
    expense_account_code: Option<String>,
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl GroupsListAssetsResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn default_useful_life_months(mut self, value: i64) -> Self {
        self.default_useful_life_months = Some(value);
        self
    }

    pub fn asset_account_code(mut self, value: impl Into<String>) -> Self {
        self.asset_account_code = Some(value.into());
        self
    }

    pub fn depreciation_account_code(mut self, value: impl Into<String>) -> Self {
        self.depreciation_account_code = Some(value.into());
        self
    }

    pub fn expense_account_code(mut self, value: impl Into<String>) -> Self {
        self.expense_account_code = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupsListAssetsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](GroupsListAssetsResponseRowsItemBuilder::code)
    /// - [`name`](GroupsListAssetsResponseRowsItemBuilder::name)
    /// - [`asset_account_code`](GroupsListAssetsResponseRowsItemBuilder::asset_account_code)
    /// - [`depreciation_account_code`](GroupsListAssetsResponseRowsItemBuilder::depreciation_account_code)
    /// - [`expense_account_code`](GroupsListAssetsResponseRowsItemBuilder::expense_account_code)
    /// - [`id`](GroupsListAssetsResponseRowsItemBuilder::id)
    /// - [`created_at`](GroupsListAssetsResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<GroupsListAssetsResponseRowsItem, BuildError> {
        Ok(GroupsListAssetsResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            default_useful_life_months: self.default_useful_life_months,
            asset_account_code: self
                .asset_account_code
                .ok_or_else(|| BuildError::missing_field("asset_account_code"))?,
            depreciation_account_code: self
                .depreciation_account_code
                .ok_or_else(|| BuildError::missing_field("depreciation_account_code"))?,
            expense_account_code: self
                .expense_account_code
                .ok_or_else(|| BuildError::missing_field("expense_account_code"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
