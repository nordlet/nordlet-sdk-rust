pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateRequestAssetGroupsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "assetAccountCode")]
    #[serde(default)]
    pub asset_account_code: String,
    #[serde(rename = "depreciationAccountCode")]
    #[serde(default)]
    pub depreciation_account_code: String,
    #[serde(rename = "expenseAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_code: Option<String>,
    #[serde(rename = "defaultUsefulLifeMonths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_useful_life_months: Option<i64>,
}

impl PostV1MigrationBooksValidateRequestAssetGroupsItem {
    pub fn builder() -> PostV1MigrationBooksValidateRequestAssetGroupsItemBuilder {
        <PostV1MigrationBooksValidateRequestAssetGroupsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateRequestAssetGroupsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    asset_account_code: Option<String>,
    depreciation_account_code: Option<String>,
    expense_account_code: Option<String>,
    default_useful_life_months: Option<i64>,
}

impl PostV1MigrationBooksValidateRequestAssetGroupsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn default_useful_life_months(mut self, value: i64) -> Self {
        self.default_useful_life_months = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateRequestAssetGroupsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1MigrationBooksValidateRequestAssetGroupsItemBuilder::code)
    /// - [`name`](PostV1MigrationBooksValidateRequestAssetGroupsItemBuilder::name)
    /// - [`asset_account_code`](PostV1MigrationBooksValidateRequestAssetGroupsItemBuilder::asset_account_code)
    /// - [`depreciation_account_code`](PostV1MigrationBooksValidateRequestAssetGroupsItemBuilder::depreciation_account_code)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateRequestAssetGroupsItem, BuildError> {
        Ok(PostV1MigrationBooksValidateRequestAssetGroupsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            asset_account_code: self
                .asset_account_code
                .ok_or_else(|| BuildError::missing_field("asset_account_code"))?,
            depreciation_account_code: self
                .depreciation_account_code
                .ok_or_else(|| BuildError::missing_field("depreciation_account_code"))?,
            expense_account_code: self.expense_account_code,
            default_useful_life_months: self.default_useful_life_months,
        })
    }
}
