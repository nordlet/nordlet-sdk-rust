pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1ReportsSizeCategoryResponseThresholdsValue {
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

impl PostV1ReportsSizeCategoryResponseThresholdsValue {
    pub fn builder() -> PostV1ReportsSizeCategoryResponseThresholdsValueBuilder {
        <PostV1ReportsSizeCategoryResponseThresholdsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsSizeCategoryResponseThresholdsValueBuilder {
    total_assets: Option<f64>,
    net_turnover: Option<f64>,
    employees: Option<f64>,
}

impl PostV1ReportsSizeCategoryResponseThresholdsValueBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsSizeCategoryResponseThresholdsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_assets`](PostV1ReportsSizeCategoryResponseThresholdsValueBuilder::total_assets)
    /// - [`net_turnover`](PostV1ReportsSizeCategoryResponseThresholdsValueBuilder::net_turnover)
    /// - [`employees`](PostV1ReportsSizeCategoryResponseThresholdsValueBuilder::employees)
    pub fn build(self) -> Result<PostV1ReportsSizeCategoryResponseThresholdsValue, BuildError> {
        Ok(PostV1ReportsSizeCategoryResponseThresholdsValue {
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
