pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsGroupsCreateRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_code: Option<String>,
}

impl PostV1AssetsGroupsCreateRequest {
    pub fn builder() -> PostV1AssetsGroupsCreateRequestBuilder {
        <PostV1AssetsGroupsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsGroupsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    default_useful_life_months: Option<i64>,
    asset_account_code: Option<String>,
    depreciation_account_code: Option<String>,
    expense_account_code: Option<String>,
}

impl PostV1AssetsGroupsCreateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AssetsGroupsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1AssetsGroupsCreateRequestBuilder::code)
    /// - [`name`](PostV1AssetsGroupsCreateRequestBuilder::name)
    /// - [`asset_account_code`](PostV1AssetsGroupsCreateRequestBuilder::asset_account_code)
    /// - [`depreciation_account_code`](PostV1AssetsGroupsCreateRequestBuilder::depreciation_account_code)
    pub fn build(self) -> Result<PostV1AssetsGroupsCreateRequest, BuildError> {
        Ok(PostV1AssetsGroupsCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            default_useful_life_months: self.default_useful_life_months,
            asset_account_code: self
                .asset_account_code
                .ok_or_else(|| BuildError::missing_field("asset_account_code"))?,
            depreciation_account_code: self
                .depreciation_account_code
                .ok_or_else(|| BuildError::missing_field("depreciation_account_code"))?,
            expense_account_code: self.expense_account_code,
        })
    }
}
