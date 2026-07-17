pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsCreateRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "acquisitionDate")]
    #[serde(default)]
    pub acquisition_date: String,
    #[serde(rename = "depreciationStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciation_start_date: Option<String>,
    #[serde(rename = "acquisitionCost")]
    #[serde(default)]
    pub acquisition_cost: String,
    #[serde(rename = "salvageValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salvage_value: Option<String>,
    #[serde(rename = "usefulLifeMonths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useful_life_months: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1AssetsAssetsCreateRequest {
    pub fn builder() -> PostV1AssetsAssetsCreateRequestBuilder {
        <PostV1AssetsAssetsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsCreateRequestBuilder {
    group_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    acquisition_date: Option<String>,
    depreciation_start_date: Option<String>,
    acquisition_cost: Option<String>,
    salvage_value: Option<String>,
    useful_life_months: Option<i64>,
    notes: Option<String>,
}

impl PostV1AssetsAssetsCreateRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
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

    pub fn acquisition_date(mut self, value: impl Into<String>) -> Self {
        self.acquisition_date = Some(value.into());
        self
    }

    pub fn depreciation_start_date(mut self, value: impl Into<String>) -> Self {
        self.depreciation_start_date = Some(value.into());
        self
    }

    pub fn acquisition_cost(mut self, value: impl Into<String>) -> Self {
        self.acquisition_cost = Some(value.into());
        self
    }

    pub fn salvage_value(mut self, value: impl Into<String>) -> Self {
        self.salvage_value = Some(value.into());
        self
    }

    pub fn useful_life_months(mut self, value: i64) -> Self {
        self.useful_life_months = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1AssetsAssetsCreateRequestBuilder::group_id)
    /// - [`code`](PostV1AssetsAssetsCreateRequestBuilder::code)
    /// - [`name`](PostV1AssetsAssetsCreateRequestBuilder::name)
    /// - [`acquisition_date`](PostV1AssetsAssetsCreateRequestBuilder::acquisition_date)
    /// - [`acquisition_cost`](PostV1AssetsAssetsCreateRequestBuilder::acquisition_cost)
    pub fn build(self) -> Result<PostV1AssetsAssetsCreateRequest, BuildError> {
        Ok(PostV1AssetsAssetsCreateRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            acquisition_date: self
                .acquisition_date
                .ok_or_else(|| BuildError::missing_field("acquisition_date"))?,
            depreciation_start_date: self.depreciation_start_date,
            acquisition_cost: self
                .acquisition_cost
                .ok_or_else(|| BuildError::missing_field("acquisition_cost"))?,
            salvage_value: self.salvage_value,
            useful_life_months: self.useful_life_months,
            notes: self.notes,
        })
    }
}
