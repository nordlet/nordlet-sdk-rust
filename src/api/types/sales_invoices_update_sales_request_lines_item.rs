pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvoicesUpdateSalesRequestLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<InvoicesUpdateSalesRequestLinesItemQuantity>,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_excl_vat: Option<String>,
    #[serde(rename = "unitPriceInclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_incl_vat: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
    #[serde(rename = "vatClassifierCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_classifier_code: Option<String>,
    #[serde(rename = "costCenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition: Option<InvoicesUpdateSalesRequestLinesItemRecognition>,
    #[serde(rename = "vatExemptionBasis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_exemption_basis: Option<String>,
    #[serde(rename = "standaloneSellingPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standalone_selling_price: Option<String>,
    #[serde(rename = "refundEstimatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_estimate_percent: Option<String>,
}

impl InvoicesUpdateSalesRequestLinesItem {
    pub fn builder() -> InvoicesUpdateSalesRequestLinesItemBuilder {
        <InvoicesUpdateSalesRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesUpdateSalesRequestLinesItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<InvoicesUpdateSalesRequestLinesItemQuantity>,
    unit_price_excl_vat: Option<String>,
    unit_price_incl_vat: Option<String>,
    vat_rate_percent: Option<String>,
    vat_classifier_code: Option<String>,
    cost_center_id: Option<String>,
    project_id: Option<String>,
    recognition: Option<InvoicesUpdateSalesRequestLinesItemRecognition>,
    vat_exemption_basis: Option<String>,
    standalone_selling_price: Option<String>,
    refund_estimate_percent: Option<String>,
}

impl InvoicesUpdateSalesRequestLinesItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: InvoicesUpdateSalesRequestLinesItemQuantity) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn unit_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_excl_vat = Some(value.into());
        self
    }

    pub fn unit_price_incl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_incl_vat = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn vat_classifier_code(mut self, value: impl Into<String>) -> Self {
        self.vat_classifier_code = Some(value.into());
        self
    }

    pub fn cost_center_id(mut self, value: impl Into<String>) -> Self {
        self.cost_center_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn recognition(mut self, value: InvoicesUpdateSalesRequestLinesItemRecognition) -> Self {
        self.recognition = Some(value);
        self
    }

    pub fn vat_exemption_basis(mut self, value: impl Into<String>) -> Self {
        self.vat_exemption_basis = Some(value.into());
        self
    }

    pub fn standalone_selling_price(mut self, value: impl Into<String>) -> Self {
        self.standalone_selling_price = Some(value.into());
        self
    }

    pub fn refund_estimate_percent(mut self, value: impl Into<String>) -> Self {
        self.refund_estimate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesUpdateSalesRequestLinesItem`].
    pub fn build(self) -> Result<InvoicesUpdateSalesRequestLinesItem, BuildError> {
        Ok(InvoicesUpdateSalesRequestLinesItem {
            item_id: self.item_id,
            description: self.description,
            unit: self.unit,
            quantity: self.quantity,
            unit_price_excl_vat: self.unit_price_excl_vat,
            unit_price_incl_vat: self.unit_price_incl_vat,
            vat_rate_percent: self.vat_rate_percent,
            vat_classifier_code: self.vat_classifier_code,
            cost_center_id: self.cost_center_id,
            project_id: self.project_id,
            recognition: self.recognition,
            vat_exemption_basis: self.vat_exemption_basis,
            standalone_selling_price: self.standalone_selling_price,
            refund_estimate_percent: self.refund_estimate_percent,
        })
    }
}
