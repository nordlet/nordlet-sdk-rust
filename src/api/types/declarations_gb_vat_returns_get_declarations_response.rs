pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatReturnsGetDeclarationsResponse {
    #[serde(rename = "periodKey")]
    #[serde(default)]
    pub period_key: String,
    #[serde(rename = "vatDueSales")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub vat_due_sales: f64,
    #[serde(rename = "vatDueAcquisitions")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub vat_due_acquisitions: f64,
    #[serde(rename = "totalVatDue")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_vat_due: f64,
    #[serde(rename = "vatReclaimedCurrPeriod")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub vat_reclaimed_curr_period: f64,
    #[serde(rename = "netVatDue")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub net_vat_due: f64,
    #[serde(rename = "totalValueSalesExVAT")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_value_sales_ex_vat: f64,
    #[serde(rename = "totalValuePurchasesExVAT")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_value_purchases_ex_vat: f64,
    #[serde(rename = "totalValueGoodsSuppliedExVAT")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_value_goods_supplied_ex_vat: f64,
    #[serde(rename = "totalAcquisitionsExVAT")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_acquisitions_ex_vat: f64,
}

impl GbVatReturnsGetDeclarationsResponse {
    pub fn builder() -> GbVatReturnsGetDeclarationsResponseBuilder {
        <GbVatReturnsGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatReturnsGetDeclarationsResponseBuilder {
    period_key: Option<String>,
    vat_due_sales: Option<f64>,
    vat_due_acquisitions: Option<f64>,
    total_vat_due: Option<f64>,
    vat_reclaimed_curr_period: Option<f64>,
    net_vat_due: Option<f64>,
    total_value_sales_ex_vat: Option<f64>,
    total_value_purchases_ex_vat: Option<f64>,
    total_value_goods_supplied_ex_vat: Option<f64>,
    total_acquisitions_ex_vat: Option<f64>,
}

impl GbVatReturnsGetDeclarationsResponseBuilder {
    pub fn period_key(mut self, value: impl Into<String>) -> Self {
        self.period_key = Some(value.into());
        self
    }

    pub fn vat_due_sales(mut self, value: f64) -> Self {
        self.vat_due_sales = Some(value);
        self
    }

    pub fn vat_due_acquisitions(mut self, value: f64) -> Self {
        self.vat_due_acquisitions = Some(value);
        self
    }

    pub fn total_vat_due(mut self, value: f64) -> Self {
        self.total_vat_due = Some(value);
        self
    }

    pub fn vat_reclaimed_curr_period(mut self, value: f64) -> Self {
        self.vat_reclaimed_curr_period = Some(value);
        self
    }

    pub fn net_vat_due(mut self, value: f64) -> Self {
        self.net_vat_due = Some(value);
        self
    }

    pub fn total_value_sales_ex_vat(mut self, value: f64) -> Self {
        self.total_value_sales_ex_vat = Some(value);
        self
    }

    pub fn total_value_purchases_ex_vat(mut self, value: f64) -> Self {
        self.total_value_purchases_ex_vat = Some(value);
        self
    }

    pub fn total_value_goods_supplied_ex_vat(mut self, value: f64) -> Self {
        self.total_value_goods_supplied_ex_vat = Some(value);
        self
    }

    pub fn total_acquisitions_ex_vat(mut self, value: f64) -> Self {
        self.total_acquisitions_ex_vat = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatReturnsGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_key`](GbVatReturnsGetDeclarationsResponseBuilder::period_key)
    /// - [`vat_due_sales`](GbVatReturnsGetDeclarationsResponseBuilder::vat_due_sales)
    /// - [`vat_due_acquisitions`](GbVatReturnsGetDeclarationsResponseBuilder::vat_due_acquisitions)
    /// - [`total_vat_due`](GbVatReturnsGetDeclarationsResponseBuilder::total_vat_due)
    /// - [`vat_reclaimed_curr_period`](GbVatReturnsGetDeclarationsResponseBuilder::vat_reclaimed_curr_period)
    /// - [`net_vat_due`](GbVatReturnsGetDeclarationsResponseBuilder::net_vat_due)
    /// - [`total_value_sales_ex_vat`](GbVatReturnsGetDeclarationsResponseBuilder::total_value_sales_ex_vat)
    /// - [`total_value_purchases_ex_vat`](GbVatReturnsGetDeclarationsResponseBuilder::total_value_purchases_ex_vat)
    /// - [`total_value_goods_supplied_ex_vat`](GbVatReturnsGetDeclarationsResponseBuilder::total_value_goods_supplied_ex_vat)
    /// - [`total_acquisitions_ex_vat`](GbVatReturnsGetDeclarationsResponseBuilder::total_acquisitions_ex_vat)
    pub fn build(self) -> Result<GbVatReturnsGetDeclarationsResponse, BuildError> {
        Ok(GbVatReturnsGetDeclarationsResponse {
            period_key: self
                .period_key
                .ok_or_else(|| BuildError::missing_field("period_key"))?,
            vat_due_sales: self
                .vat_due_sales
                .ok_or_else(|| BuildError::missing_field("vat_due_sales"))?,
            vat_due_acquisitions: self
                .vat_due_acquisitions
                .ok_or_else(|| BuildError::missing_field("vat_due_acquisitions"))?,
            total_vat_due: self
                .total_vat_due
                .ok_or_else(|| BuildError::missing_field("total_vat_due"))?,
            vat_reclaimed_curr_period: self
                .vat_reclaimed_curr_period
                .ok_or_else(|| BuildError::missing_field("vat_reclaimed_curr_period"))?,
            net_vat_due: self
                .net_vat_due
                .ok_or_else(|| BuildError::missing_field("net_vat_due"))?,
            total_value_sales_ex_vat: self
                .total_value_sales_ex_vat
                .ok_or_else(|| BuildError::missing_field("total_value_sales_ex_vat"))?,
            total_value_purchases_ex_vat: self
                .total_value_purchases_ex_vat
                .ok_or_else(|| BuildError::missing_field("total_value_purchases_ex_vat"))?,
            total_value_goods_supplied_ex_vat: self
                .total_value_goods_supplied_ex_vat
                .ok_or_else(|| BuildError::missing_field("total_value_goods_supplied_ex_vat"))?,
            total_acquisitions_ex_vat: self
                .total_acquisitions_ex_vat
                .ok_or_else(|| BuildError::missing_field("total_acquisitions_ex_vat"))?,
        })
    }
}
