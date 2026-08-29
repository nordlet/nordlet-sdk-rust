pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesOrdersRejectResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "receivedQty")]
    #[serde(default)]
    pub received_qty: String,
    #[serde(rename = "remainingQty")]
    #[serde(default)]
    pub remaining_qty: String,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_excl_vat: Option<String>,
    #[serde(rename = "unitPriceInclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_incl_vat: Option<String>,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(rename = "vatClassifierCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_classifier_code: Option<String>,
    #[serde(rename = "costCenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "lineNet")]
    #[serde(default)]
    pub line_net: String,
    #[serde(rename = "lineVat")]
    #[serde(default)]
    pub line_vat: String,
    #[serde(rename = "lineGross")]
    #[serde(default)]
    pub line_gross: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
}

impl PostV1PurchasesOrdersRejectResponseLinesItem {
    pub fn builder() -> PostV1PurchasesOrdersRejectResponseLinesItemBuilder {
        <PostV1PurchasesOrdersRejectResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesOrdersRejectResponseLinesItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<String>,
    received_qty: Option<String>,
    remaining_qty: Option<String>,
    unit_price_excl_vat: Option<String>,
    unit_price_incl_vat: Option<String>,
    vat_rate_percent: Option<String>,
    vat_classifier_code: Option<String>,
    cost_center_id: Option<String>,
    project_id: Option<String>,
    account_code: Option<String>,
    line_net: Option<String>,
    line_vat: Option<String>,
    line_gross: Option<String>,
    sort_order: Option<i64>,
}

impl PostV1PurchasesOrdersRejectResponseLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn received_qty(mut self, value: impl Into<String>) -> Self {
        self.received_qty = Some(value.into());
        self
    }

    pub fn remaining_qty(mut self, value: impl Into<String>) -> Self {
        self.remaining_qty = Some(value.into());
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

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn line_net(mut self, value: impl Into<String>) -> Self {
        self.line_net = Some(value.into());
        self
    }

    pub fn line_vat(mut self, value: impl Into<String>) -> Self {
        self.line_vat = Some(value.into());
        self
    }

    pub fn line_gross(mut self, value: impl Into<String>) -> Self {
        self.line_gross = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesOrdersRejectResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::id)
    /// - [`description`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::description)
    /// - [`unit`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::unit)
    /// - [`quantity`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::quantity)
    /// - [`received_qty`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::received_qty)
    /// - [`remaining_qty`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::remaining_qty)
    /// - [`vat_rate_percent`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::vat_rate_percent)
    /// - [`line_net`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::line_net)
    /// - [`line_vat`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::line_vat)
    /// - [`line_gross`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::line_gross)
    /// - [`sort_order`](PostV1PurchasesOrdersRejectResponseLinesItemBuilder::sort_order)
    pub fn build(self) -> Result<PostV1PurchasesOrdersRejectResponseLinesItem, BuildError> {
        Ok(PostV1PurchasesOrdersRejectResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            received_qty: self
                .received_qty
                .ok_or_else(|| BuildError::missing_field("received_qty"))?,
            remaining_qty: self
                .remaining_qty
                .ok_or_else(|| BuildError::missing_field("remaining_qty"))?,
            unit_price_excl_vat: self.unit_price_excl_vat,
            unit_price_incl_vat: self.unit_price_incl_vat,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            vat_classifier_code: self.vat_classifier_code,
            cost_center_id: self.cost_center_id,
            project_id: self.project_id,
            account_code: self.account_code,
            line_net: self
                .line_net
                .ok_or_else(|| BuildError::missing_field("line_net"))?,
            line_vat: self
                .line_vat
                .ok_or_else(|| BuildError::missing_field("line_vat"))?,
            line_gross: self
                .line_gross
                .ok_or_else(|| BuildError::missing_field("line_gross"))?,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
        })
    }
}
