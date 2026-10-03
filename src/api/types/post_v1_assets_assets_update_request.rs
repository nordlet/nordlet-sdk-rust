pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsUpdateRequest {
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "acquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<String>,
    #[serde(rename = "depreciationStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciation_start_date: Option<String>,
    #[serde(rename = "acquisitionCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_cost: Option<String>,
    #[serde(rename = "salvageValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salvage_value: Option<String>,
    #[serde(rename = "usefulLifeMonths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useful_life_months: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<PostV1AssetsAssetsUpdateRequestDocumentsItem>>,
    #[serde(default)]
    pub id: String,
}

impl PostV1AssetsAssetsUpdateRequest {
    pub fn builder() -> PostV1AssetsAssetsUpdateRequestBuilder {
        <PostV1AssetsAssetsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsUpdateRequestBuilder {
    group_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    acquisition_date: Option<String>,
    depreciation_start_date: Option<String>,
    acquisition_cost: Option<String>,
    salvage_value: Option<String>,
    useful_life_months: Option<i64>,
    notes: Option<String>,
    documents: Option<Vec<PostV1AssetsAssetsUpdateRequestDocumentsItem>>,
    id: Option<String>,
}

impl PostV1AssetsAssetsUpdateRequestBuilder {
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

    pub fn documents(mut self, value: Vec<PostV1AssetsAssetsUpdateRequestDocumentsItem>) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AssetsAssetsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AssetsAssetsUpdateRequest, BuildError> {
        Ok(PostV1AssetsAssetsUpdateRequest {
            group_id: self.group_id,
            code: self.code,
            name: self.name,
            acquisition_date: self.acquisition_date,
            depreciation_start_date: self.depreciation_start_date,
            acquisition_cost: self.acquisition_cost,
            salvage_value: self.salvage_value,
            useful_life_months: self.useful_life_months,
            notes: self.notes,
            documents: self.documents,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
