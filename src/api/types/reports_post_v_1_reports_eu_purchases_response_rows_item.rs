pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsEuPurchasesResponseRowsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub documents: i64,
}

impl PostV1ReportsEuPurchasesResponseRowsItem {
    pub fn builder() -> PostV1ReportsEuPurchasesResponseRowsItemBuilder {
        <PostV1ReportsEuPurchasesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsEuPurchasesResponseRowsItemBuilder {
    country_code: Option<String>,
    vat_rate_percent: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    documents: Option<i64>,
}

impl PostV1ReportsEuPurchasesResponseRowsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsEuPurchasesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1ReportsEuPurchasesResponseRowsItemBuilder::country_code)
    /// - [`vat_rate_percent`](PostV1ReportsEuPurchasesResponseRowsItemBuilder::vat_rate_percent)
    /// - [`net`](PostV1ReportsEuPurchasesResponseRowsItemBuilder::net)
    /// - [`vat`](PostV1ReportsEuPurchasesResponseRowsItemBuilder::vat)
    /// - [`documents`](PostV1ReportsEuPurchasesResponseRowsItemBuilder::documents)
    pub fn build(self) -> Result<PostV1ReportsEuPurchasesResponseRowsItem, BuildError> {
        Ok(PostV1ReportsEuPurchasesResponseRowsItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
