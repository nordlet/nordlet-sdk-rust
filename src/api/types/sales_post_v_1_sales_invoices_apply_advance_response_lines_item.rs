pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesApplyAdvanceResponseLinesItem {
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
    #[serde(rename = "recognitionMethod")]
    pub recognition_method: PostV1SalesInvoicesApplyAdvanceResponseLinesItemRecognitionMethod,
    #[serde(rename = "recognitionStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_start_date: Option<String>,
    #[serde(rename = "recognitionEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_end_date: Option<String>,
    #[serde(rename = "recognitionMilestones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_milestones:
        Option<Vec<PostV1SalesInvoicesApplyAdvanceResponseLinesItemRecognitionMilestonesItem>>,
    #[serde(rename = "standaloneSellingPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standalone_selling_price: Option<String>,
    #[serde(rename = "allocatedNet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_net: Option<String>,
    #[serde(rename = "refundEstimatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_estimate_percent: Option<String>,
}

impl PostV1SalesInvoicesApplyAdvanceResponseLinesItem {
    pub fn builder() -> PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder {
        <PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder {
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
    recognition_method: Option<PostV1SalesInvoicesApplyAdvanceResponseLinesItemRecognitionMethod>,
    recognition_start_date: Option<String>,
    recognition_end_date: Option<String>,
    recognition_milestones:
        Option<Vec<PostV1SalesInvoicesApplyAdvanceResponseLinesItemRecognitionMilestonesItem>>,
    standalone_selling_price: Option<String>,
    allocated_net: Option<String>,
    refund_estimate_percent: Option<String>,
}

impl PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder {
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

    pub fn recognition_method(
        mut self,
        value: PostV1SalesInvoicesApplyAdvanceResponseLinesItemRecognitionMethod,
    ) -> Self {
        self.recognition_method = Some(value);
        self
    }

    pub fn recognition_start_date(mut self, value: impl Into<String>) -> Self {
        self.recognition_start_date = Some(value.into());
        self
    }

    pub fn recognition_end_date(mut self, value: impl Into<String>) -> Self {
        self.recognition_end_date = Some(value.into());
        self
    }

    pub fn recognition_milestones(
        mut self,
        value: Vec<PostV1SalesInvoicesApplyAdvanceResponseLinesItemRecognitionMilestonesItem>,
    ) -> Self {
        self.recognition_milestones = Some(value);
        self
    }

    pub fn standalone_selling_price(mut self, value: impl Into<String>) -> Self {
        self.standalone_selling_price = Some(value.into());
        self
    }

    pub fn allocated_net(mut self, value: impl Into<String>) -> Self {
        self.allocated_net = Some(value.into());
        self
    }

    pub fn refund_estimate_percent(mut self, value: impl Into<String>) -> Self {
        self.refund_estimate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesApplyAdvanceResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::id)
    /// - [`description`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::description)
    /// - [`unit`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::unit)
    /// - [`quantity`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::quantity)
    /// - [`vat_rate_percent`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::vat_rate_percent)
    /// - [`line_net`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::line_net)
    /// - [`line_vat`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::line_vat)
    /// - [`line_gross`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::line_gross)
    /// - [`sort_order`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::sort_order)
    /// - [`recognition_method`](PostV1SalesInvoicesApplyAdvanceResponseLinesItemBuilder::recognition_method)
    pub fn build(self) -> Result<PostV1SalesInvoicesApplyAdvanceResponseLinesItem, BuildError> {
        Ok(PostV1SalesInvoicesApplyAdvanceResponseLinesItem {
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
            recognition_method: self
                .recognition_method
                .ok_or_else(|| BuildError::missing_field("recognition_method"))?,
            recognition_start_date: self.recognition_start_date,
            recognition_end_date: self.recognition_end_date,
            recognition_milestones: self.recognition_milestones,
            standalone_selling_price: self.standalone_selling_price,
            allocated_net: self.allocated_net,
            refund_estimate_percent: self.refund_estimate_percent,
        })
    }
}
