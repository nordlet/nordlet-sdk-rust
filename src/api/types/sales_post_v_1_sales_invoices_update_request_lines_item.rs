pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1SalesInvoicesUpdateRequestLinesItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<PostV1SalesInvoicesUpdateRequestLinesItemQuantity>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition: Option<PostV1SalesInvoicesUpdateRequestLinesItemRecognition>,
    #[serde(rename = "standaloneSellingPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standalone_selling_price: Option<String>,
    #[serde(rename = "refundEstimatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_estimate_percent: Option<String>,
}

impl PostV1SalesInvoicesUpdateRequestLinesItem {
    pub fn builder() -> PostV1SalesInvoicesUpdateRequestLinesItemBuilder {
        <PostV1SalesInvoicesUpdateRequestLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUpdateRequestLinesItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<PostV1SalesInvoicesUpdateRequestLinesItemQuantity>,
    unit_price_excl_vat: Option<String>,
    unit_price_incl_vat: Option<String>,
    vat_rate_percent: Option<String>,
    vat_classifier_code: Option<String>,
    cost_center_id: Option<String>,
    recognition: Option<PostV1SalesInvoicesUpdateRequestLinesItemRecognition>,
    standalone_selling_price: Option<String>,
    refund_estimate_percent: Option<String>,
}

impl PostV1SalesInvoicesUpdateRequestLinesItemBuilder {
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

    pub fn quantity(mut self, value: PostV1SalesInvoicesUpdateRequestLinesItemQuantity) -> Self {
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

    pub fn recognition(
        mut self,
        value: PostV1SalesInvoicesUpdateRequestLinesItemRecognition,
    ) -> Self {
        self.recognition = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUpdateRequestLinesItem`].
    pub fn build(self) -> Result<PostV1SalesInvoicesUpdateRequestLinesItem, BuildError> {
        Ok(PostV1SalesInvoicesUpdateRequestLinesItem {
            item_id: self.item_id,
            description: self.description,
            unit: self.unit,
            quantity: self.quantity,
            unit_price_excl_vat: self.unit_price_excl_vat,
            unit_price_incl_vat: self.unit_price_incl_vat,
            vat_rate_percent: self.vat_rate_percent,
            vat_classifier_code: self.vat_classifier_code,
            cost_center_id: self.cost_center_id,
            recognition: self.recognition,
            standalone_selling_price: self.standalone_selling_price,
            refund_estimate_percent: self.refund_estimate_percent,
        })
    }
}
