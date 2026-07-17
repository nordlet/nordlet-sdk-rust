pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesCreateResponseLinesItem {
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

impl PostV1SalesInvoicesCreateResponseLinesItem {
    pub fn builder() -> PostV1SalesInvoicesCreateResponseLinesItemBuilder {
        <PostV1SalesInvoicesCreateResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesCreateResponseLinesItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    description: Option<String>,
    unit: Option<String>,
    quantity: Option<String>,
    unit_price_excl_vat: Option<String>,
    unit_price_incl_vat: Option<String>,
    vat_rate_percent: Option<String>,
    vat_classifier_code: Option<String>,
    cost_center_id: Option<String>,
    line_net: Option<String>,
    line_vat: Option<String>,
    line_gross: Option<String>,
    sort_order: Option<i64>,
}

impl PostV1SalesInvoicesCreateResponseLinesItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesCreateResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::id)
    /// - [`description`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::description)
    /// - [`unit`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::unit)
    /// - [`quantity`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::quantity)
    /// - [`vat_rate_percent`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::vat_rate_percent)
    /// - [`line_net`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::line_net)
    /// - [`line_vat`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::line_vat)
    /// - [`line_gross`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::line_gross)
    /// - [`sort_order`](PostV1SalesInvoicesCreateResponseLinesItemBuilder::sort_order)
    pub fn build(self) -> Result<PostV1SalesInvoicesCreateResponseLinesItem, BuildError> {
        Ok(PostV1SalesInvoicesCreateResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price_excl_vat: self.unit_price_excl_vat,
            unit_price_incl_vat: self.unit_price_incl_vat,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            vat_classifier_code: self.vat_classifier_code,
            cost_center_id: self.cost_center_id,
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
