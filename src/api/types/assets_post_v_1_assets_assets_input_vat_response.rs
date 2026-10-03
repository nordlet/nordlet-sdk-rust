pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsInputVatResponse {
    #[serde(default)]
    pub id: String,
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
    #[serde(default)]
    pub depreciation_start_date: String,
    #[serde(rename = "acquisitionCost")]
    #[serde(default)]
    pub acquisition_cost: String,
    #[serde(rename = "salvageValue")]
    #[serde(default)]
    pub salvage_value: String,
    #[serde(rename = "usefulLifeMonths")]
    #[serde(default)]
    pub useful_life_months: i64,
    #[serde(rename = "totalCost")]
    #[serde(default)]
    pub total_cost: String,
    #[serde(rename = "accumulatedDepreciation")]
    #[serde(default)]
    pub accumulated_depreciation: String,
    #[serde(rename = "netBookValue")]
    #[serde(default)]
    pub net_book_value: String,
    #[serde(rename = "depreciatedMonths")]
    #[serde(default)]
    pub depreciated_months: i64,
    #[serde(rename = "totalLifeMonths")]
    #[serde(default)]
    pub total_life_months: i64,
    pub status: PostV1AssetsAssetsInputVatResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<PostV1AssetsAssetsInputVatResponseDocumentsItem>>,
    #[serde(rename = "inputVatAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_vat_amount: Option<String>,
    #[serde(rename = "inputVatFirstUseDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_vat_first_use_date: Option<String>,
    #[serde(rename = "inputVatDeductiblePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_vat_deductible_percent: Option<String>,
    #[serde(rename = "inputVatRealEstate")]
    #[serde(default)]
    pub input_vat_real_estate: bool,
    #[serde(rename = "inputVatUseChanges")]
    #[serde(default)]
    pub input_vat_use_changes: Vec<PostV1AssetsAssetsInputVatResponseInputVatUseChangesItem>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1AssetsAssetsInputVatResponse {
    pub fn builder() -> PostV1AssetsAssetsInputVatResponseBuilder {
        <PostV1AssetsAssetsInputVatResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsInputVatResponseBuilder {
    id: Option<String>,
    group_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    acquisition_date: Option<String>,
    depreciation_start_date: Option<String>,
    acquisition_cost: Option<String>,
    salvage_value: Option<String>,
    useful_life_months: Option<i64>,
    total_cost: Option<String>,
    accumulated_depreciation: Option<String>,
    net_book_value: Option<String>,
    depreciated_months: Option<i64>,
    total_life_months: Option<i64>,
    status: Option<PostV1AssetsAssetsInputVatResponseStatus>,
    notes: Option<String>,
    documents: Option<Vec<PostV1AssetsAssetsInputVatResponseDocumentsItem>>,
    input_vat_amount: Option<String>,
    input_vat_first_use_date: Option<String>,
    input_vat_deductible_percent: Option<String>,
    input_vat_real_estate: Option<bool>,
    input_vat_use_changes: Option<Vec<PostV1AssetsAssetsInputVatResponseInputVatUseChangesItem>>,
    created_at: Option<String>,
}

impl PostV1AssetsAssetsInputVatResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn total_cost(mut self, value: impl Into<String>) -> Self {
        self.total_cost = Some(value.into());
        self
    }

    pub fn accumulated_depreciation(mut self, value: impl Into<String>) -> Self {
        self.accumulated_depreciation = Some(value.into());
        self
    }

    pub fn net_book_value(mut self, value: impl Into<String>) -> Self {
        self.net_book_value = Some(value.into());
        self
    }

    pub fn depreciated_months(mut self, value: i64) -> Self {
        self.depreciated_months = Some(value);
        self
    }

    pub fn total_life_months(mut self, value: i64) -> Self {
        self.total_life_months = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1AssetsAssetsInputVatResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn documents(
        mut self,
        value: Vec<PostV1AssetsAssetsInputVatResponseDocumentsItem>,
    ) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn input_vat_amount(mut self, value: impl Into<String>) -> Self {
        self.input_vat_amount = Some(value.into());
        self
    }

    pub fn input_vat_first_use_date(mut self, value: impl Into<String>) -> Self {
        self.input_vat_first_use_date = Some(value.into());
        self
    }

    pub fn input_vat_deductible_percent(mut self, value: impl Into<String>) -> Self {
        self.input_vat_deductible_percent = Some(value.into());
        self
    }

    pub fn input_vat_real_estate(mut self, value: bool) -> Self {
        self.input_vat_real_estate = Some(value);
        self
    }

    pub fn input_vat_use_changes(
        mut self,
        value: Vec<PostV1AssetsAssetsInputVatResponseInputVatUseChangesItem>,
    ) -> Self {
        self.input_vat_use_changes = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsInputVatResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AssetsAssetsInputVatResponseBuilder::id)
    /// - [`group_id`](PostV1AssetsAssetsInputVatResponseBuilder::group_id)
    /// - [`code`](PostV1AssetsAssetsInputVatResponseBuilder::code)
    /// - [`name`](PostV1AssetsAssetsInputVatResponseBuilder::name)
    /// - [`acquisition_date`](PostV1AssetsAssetsInputVatResponseBuilder::acquisition_date)
    /// - [`depreciation_start_date`](PostV1AssetsAssetsInputVatResponseBuilder::depreciation_start_date)
    /// - [`acquisition_cost`](PostV1AssetsAssetsInputVatResponseBuilder::acquisition_cost)
    /// - [`salvage_value`](PostV1AssetsAssetsInputVatResponseBuilder::salvage_value)
    /// - [`useful_life_months`](PostV1AssetsAssetsInputVatResponseBuilder::useful_life_months)
    /// - [`total_cost`](PostV1AssetsAssetsInputVatResponseBuilder::total_cost)
    /// - [`accumulated_depreciation`](PostV1AssetsAssetsInputVatResponseBuilder::accumulated_depreciation)
    /// - [`net_book_value`](PostV1AssetsAssetsInputVatResponseBuilder::net_book_value)
    /// - [`depreciated_months`](PostV1AssetsAssetsInputVatResponseBuilder::depreciated_months)
    /// - [`total_life_months`](PostV1AssetsAssetsInputVatResponseBuilder::total_life_months)
    /// - [`status`](PostV1AssetsAssetsInputVatResponseBuilder::status)
    /// - [`input_vat_real_estate`](PostV1AssetsAssetsInputVatResponseBuilder::input_vat_real_estate)
    /// - [`input_vat_use_changes`](PostV1AssetsAssetsInputVatResponseBuilder::input_vat_use_changes)
    /// - [`created_at`](PostV1AssetsAssetsInputVatResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1AssetsAssetsInputVatResponse, BuildError> {
        Ok(PostV1AssetsAssetsInputVatResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            acquisition_date: self
                .acquisition_date
                .ok_or_else(|| BuildError::missing_field("acquisition_date"))?,
            depreciation_start_date: self
                .depreciation_start_date
                .ok_or_else(|| BuildError::missing_field("depreciation_start_date"))?,
            acquisition_cost: self
                .acquisition_cost
                .ok_or_else(|| BuildError::missing_field("acquisition_cost"))?,
            salvage_value: self
                .salvage_value
                .ok_or_else(|| BuildError::missing_field("salvage_value"))?,
            useful_life_months: self
                .useful_life_months
                .ok_or_else(|| BuildError::missing_field("useful_life_months"))?,
            total_cost: self
                .total_cost
                .ok_or_else(|| BuildError::missing_field("total_cost"))?,
            accumulated_depreciation: self
                .accumulated_depreciation
                .ok_or_else(|| BuildError::missing_field("accumulated_depreciation"))?,
            net_book_value: self
                .net_book_value
                .ok_or_else(|| BuildError::missing_field("net_book_value"))?,
            depreciated_months: self
                .depreciated_months
                .ok_or_else(|| BuildError::missing_field("depreciated_months"))?,
            total_life_months: self
                .total_life_months
                .ok_or_else(|| BuildError::missing_field("total_life_months"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
            documents: self.documents,
            input_vat_amount: self.input_vat_amount,
            input_vat_first_use_date: self.input_vat_first_use_date,
            input_vat_deductible_percent: self.input_vat_deductible_percent,
            input_vat_real_estate: self
                .input_vat_real_estate
                .ok_or_else(|| BuildError::missing_field("input_vat_real_estate"))?,
            input_vat_use_changes: self
                .input_vat_use_changes
                .ok_or_else(|| BuildError::missing_field("input_vat_use_changes"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
