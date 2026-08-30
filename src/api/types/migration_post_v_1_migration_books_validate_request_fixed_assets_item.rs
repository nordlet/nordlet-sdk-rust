pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateRequestFixedAssetsItem {
    #[serde(rename = "groupCode")]
    #[serde(default)]
    pub group_code: String,
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
    #[serde(rename = "accumulatedDepreciation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulated_depreciation: Option<String>,
    #[serde(rename = "depreciatedMonths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciated_months: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1MigrationBooksValidateRequestFixedAssetsItem {
    pub fn builder() -> PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder {
        <PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder {
    group_code: Option<String>,
    code: Option<String>,
    name: Option<String>,
    acquisition_date: Option<String>,
    depreciation_start_date: Option<String>,
    acquisition_cost: Option<String>,
    salvage_value: Option<String>,
    useful_life_months: Option<i64>,
    accumulated_depreciation: Option<String>,
    depreciated_months: Option<i64>,
    notes: Option<String>,
}

impl PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder {
    pub fn group_code(mut self, value: impl Into<String>) -> Self {
        self.group_code = Some(value.into());
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

    pub fn accumulated_depreciation(mut self, value: impl Into<String>) -> Self {
        self.accumulated_depreciation = Some(value.into());
        self
    }

    pub fn depreciated_months(mut self, value: i64) -> Self {
        self.depreciated_months = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateRequestFixedAssetsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_code`](PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder::group_code)
    /// - [`code`](PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder::code)
    /// - [`name`](PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder::name)
    /// - [`acquisition_date`](PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder::acquisition_date)
    /// - [`acquisition_cost`](PostV1MigrationBooksValidateRequestFixedAssetsItemBuilder::acquisition_cost)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateRequestFixedAssetsItem, BuildError> {
        Ok(PostV1MigrationBooksValidateRequestFixedAssetsItem {
            group_code: self
                .group_code
                .ok_or_else(|| BuildError::missing_field("group_code"))?,
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
            accumulated_depreciation: self.accumulated_depreciation,
            depreciated_months: self.depreciated_months,
            notes: self.notes,
        })
    }
}
