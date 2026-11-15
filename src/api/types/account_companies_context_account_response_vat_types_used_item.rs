pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesContextAccountResponseVatTypesUsedItem {
    #[serde(default)]
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ratePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_percent: Option<String>,
    #[serde(rename = "saleLines")]
    #[serde(default)]
    pub sale_lines: i64,
    #[serde(rename = "purchaseLines")]
    #[serde(default)]
    pub purchase_lines: i64,
}

impl CompaniesContextAccountResponseVatTypesUsedItem {
    pub fn builder() -> CompaniesContextAccountResponseVatTypesUsedItemBuilder {
        <CompaniesContextAccountResponseVatTypesUsedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesContextAccountResponseVatTypesUsedItemBuilder {
    code: Option<String>,
    name: Option<String>,
    rate_percent: Option<String>,
    sale_lines: Option<i64>,
    purchase_lines: Option<i64>,
}

impl CompaniesContextAccountResponseVatTypesUsedItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    pub fn sale_lines(mut self, value: i64) -> Self {
        self.sale_lines = Some(value);
        self
    }

    pub fn purchase_lines(mut self, value: i64) -> Self {
        self.purchase_lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesContextAccountResponseVatTypesUsedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CompaniesContextAccountResponseVatTypesUsedItemBuilder::code)
    /// - [`sale_lines`](CompaniesContextAccountResponseVatTypesUsedItemBuilder::sale_lines)
    /// - [`purchase_lines`](CompaniesContextAccountResponseVatTypesUsedItemBuilder::purchase_lines)
    pub fn build(self) -> Result<CompaniesContextAccountResponseVatTypesUsedItem, BuildError> {
        Ok(CompaniesContextAccountResponseVatTypesUsedItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name,
            rate_percent: self.rate_percent,
            sale_lines: self
                .sale_lines
                .ok_or_else(|| BuildError::missing_field("sale_lines"))?,
            purchase_lines: self
                .purchase_lines
                .ok_or_else(|| BuildError::missing_field("purchase_lines"))?,
        })
    }
}
