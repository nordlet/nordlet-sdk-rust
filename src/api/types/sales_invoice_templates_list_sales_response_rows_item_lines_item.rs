pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceTemplatesListSalesResponseRowsItemLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_excl_vat: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(rename = "vatExemptionBasis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_exemption_basis: Option<String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl InvoiceTemplatesListSalesResponseRowsItemLinesItem {
    pub fn builder() -> InvoiceTemplatesListSalesResponseRowsItemLinesItemBuilder {
        <InvoiceTemplatesListSalesResponseRowsItemLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceTemplatesListSalesResponseRowsItemLinesItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    unit_price_excl_vat: Option<String>,
    vat_rate_percent: Option<String>,
    vat_exemption_basis: Option<String>,
    project_id: Option<String>,
    unit: Option<String>,
}

impl InvoiceTemplatesListSalesResponseRowsItemLinesItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_excl_vat = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn vat_exemption_basis(mut self, value: impl Into<String>) -> Self {
        self.vat_exemption_basis = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceTemplatesListSalesResponseRowsItemLinesItem`].
    pub fn build(self) -> Result<InvoiceTemplatesListSalesResponseRowsItemLinesItem, BuildError> {
        Ok(InvoiceTemplatesListSalesResponseRowsItemLinesItem {
            item_id: self.item_id,
            description: self.description,
            quantity: self.quantity,
            unit_price_excl_vat: self.unit_price_excl_vat,
            vat_rate_percent: self.vat_rate_percent,
            vat_exemption_basis: self.vat_exemption_basis,
            project_id: self.project_id,
            unit: self.unit,
        })
    }
}
