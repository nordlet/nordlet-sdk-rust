pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsInputVatRequest {
    #[serde(default)]
    pub id: String,
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
    pub input_vat_use_changes: Vec<PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem>,
}

impl PostV1AssetsAssetsInputVatRequest {
    pub fn builder() -> PostV1AssetsAssetsInputVatRequestBuilder {
        <PostV1AssetsAssetsInputVatRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsInputVatRequestBuilder {
    id: Option<String>,
    input_vat_amount: Option<String>,
    input_vat_first_use_date: Option<String>,
    input_vat_deductible_percent: Option<String>,
    input_vat_real_estate: Option<bool>,
    input_vat_use_changes: Option<Vec<PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem>>,
}

impl PostV1AssetsAssetsInputVatRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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
        value: Vec<PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem>,
    ) -> Self {
        self.input_vat_use_changes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsInputVatRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AssetsAssetsInputVatRequestBuilder::id)
    /// - [`input_vat_real_estate`](PostV1AssetsAssetsInputVatRequestBuilder::input_vat_real_estate)
    /// - [`input_vat_use_changes`](PostV1AssetsAssetsInputVatRequestBuilder::input_vat_use_changes)
    pub fn build(self) -> Result<PostV1AssetsAssetsInputVatRequest, BuildError> {
        Ok(PostV1AssetsAssetsInputVatRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input_vat_amount: self.input_vat_amount,
            input_vat_first_use_date: self.input_vat_first_use_date,
            input_vat_deductible_percent: self.input_vat_deductible_percent,
            input_vat_real_estate: self
                .input_vat_real_estate
                .ok_or_else(|| BuildError::missing_field("input_vat_real_estate"))?,
            input_vat_use_changes: self
                .input_vat_use_changes
                .ok_or_else(|| BuildError::missing_field("input_vat_use_changes"))?,
        })
    }
}
