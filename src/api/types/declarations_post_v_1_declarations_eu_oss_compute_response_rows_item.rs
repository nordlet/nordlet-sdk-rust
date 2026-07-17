pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuOssComputeResponseRowsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "rateType")]
    pub rate_type: PostV1DeclarationsEuOssComputeResponseRowsItemRateType,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
    #[serde(default)]
    pub documents: i64,
}

impl PostV1DeclarationsEuOssComputeResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsEuOssComputeResponseRowsItemBuilder {
        <PostV1DeclarationsEuOssComputeResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuOssComputeResponseRowsItemBuilder {
    country_code: Option<String>,
    rate_type: Option<PostV1DeclarationsEuOssComputeResponseRowsItemRateType>,
    vat_rate_percent: Option<String>,
    taxable_amount: Option<String>,
    vat_amount: Option<String>,
    documents: Option<i64>,
}

impl PostV1DeclarationsEuOssComputeResponseRowsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn rate_type(
        mut self,
        value: PostV1DeclarationsEuOssComputeResponseRowsItemRateType,
    ) -> Self {
        self.rate_type = Some(value);
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuOssComputeResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsEuOssComputeResponseRowsItemBuilder::country_code)
    /// - [`rate_type`](PostV1DeclarationsEuOssComputeResponseRowsItemBuilder::rate_type)
    /// - [`vat_rate_percent`](PostV1DeclarationsEuOssComputeResponseRowsItemBuilder::vat_rate_percent)
    /// - [`taxable_amount`](PostV1DeclarationsEuOssComputeResponseRowsItemBuilder::taxable_amount)
    /// - [`vat_amount`](PostV1DeclarationsEuOssComputeResponseRowsItemBuilder::vat_amount)
    /// - [`documents`](PostV1DeclarationsEuOssComputeResponseRowsItemBuilder::documents)
    pub fn build(self) -> Result<PostV1DeclarationsEuOssComputeResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsEuOssComputeResponseRowsItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            rate_type: self
                .rate_type
                .ok_or_else(|| BuildError::missing_field("rate_type"))?,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
