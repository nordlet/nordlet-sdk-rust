pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SizeCategoryReportsResponseThresholdsValue {
    #[serde(rename = "totalAssets")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_assets: f64,
    #[serde(rename = "netTurnover")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub net_turnover: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub employees: f64,
}

impl SizeCategoryReportsResponseThresholdsValue {
    pub fn builder() -> SizeCategoryReportsResponseThresholdsValueBuilder {
        <SizeCategoryReportsResponseThresholdsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SizeCategoryReportsResponseThresholdsValueBuilder {
    total_assets: Option<f64>,
    net_turnover: Option<f64>,
    employees: Option<f64>,
}

impl SizeCategoryReportsResponseThresholdsValueBuilder {
    pub fn total_assets(mut self, value: f64) -> Self {
        self.total_assets = Some(value);
        self
    }

    pub fn net_turnover(mut self, value: f64) -> Self {
        self.net_turnover = Some(value);
        self
    }

    pub fn employees(mut self, value: f64) -> Self {
        self.employees = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SizeCategoryReportsResponseThresholdsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_assets`](SizeCategoryReportsResponseThresholdsValueBuilder::total_assets)
    /// - [`net_turnover`](SizeCategoryReportsResponseThresholdsValueBuilder::net_turnover)
    /// - [`employees`](SizeCategoryReportsResponseThresholdsValueBuilder::employees)
    pub fn build(self) -> Result<SizeCategoryReportsResponseThresholdsValue, BuildError> {
        Ok(SizeCategoryReportsResponseThresholdsValue {
            total_assets: self
                .total_assets
                .ok_or_else(|| BuildError::missing_field("total_assets"))?,
            net_turnover: self
                .net_turnover
                .ok_or_else(|| BuildError::missing_field("net_turnover"))?,
            employees: self
                .employees
                .ok_or_else(|| BuildError::missing_field("employees"))?,
        })
    }
}
