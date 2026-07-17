pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsGroupsCreateResponse {
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
    pub created_at: String,
}

impl PostV1AssetsGroupsCreateResponse {
    pub fn builder() -> PostV1AssetsGroupsCreateResponseBuilder {
        <PostV1AssetsGroupsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsGroupsCreateResponseBuilder {
    code: Option<String>,
    name: Option<String>,
    default_useful_life_months: Option<i64>,
    asset_account_code: Option<String>,
    depreciation_account_code: Option<String>,
    expense_account_code: Option<String>,
    id: Option<String>,
    created_at: Option<String>,
}

impl PostV1AssetsGroupsCreateResponseBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsGroupsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1AssetsGroupsCreateResponseBuilder::code)
    /// - [`name`](PostV1AssetsGroupsCreateResponseBuilder::name)
    /// - [`asset_account_code`](PostV1AssetsGroupsCreateResponseBuilder::asset_account_code)
    /// - [`depreciation_account_code`](PostV1AssetsGroupsCreateResponseBuilder::depreciation_account_code)
    /// - [`expense_account_code`](PostV1AssetsGroupsCreateResponseBuilder::expense_account_code)
    /// - [`id`](PostV1AssetsGroupsCreateResponseBuilder::id)
    /// - [`created_at`](PostV1AssetsGroupsCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1AssetsGroupsCreateResponse, BuildError> {
        Ok(PostV1AssetsGroupsCreateResponse {
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
