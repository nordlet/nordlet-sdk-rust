pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1ReportsSizeCategoryResponseCriteria {
    #[serde(rename = "totalAssets")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_assets: f64,
    #[serde(rename = "netTurnover")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub net_turnover: f64,
    #[serde(rename = "avgEmployees")]
    #[serde(default)]
    pub avg_employees: i64,
}

impl PostV1ReportsSizeCategoryResponseCriteria {
    pub fn builder() -> PostV1ReportsSizeCategoryResponseCriteriaBuilder {
        <PostV1ReportsSizeCategoryResponseCriteriaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsSizeCategoryResponseCriteriaBuilder {
    total_assets: Option<f64>,
    net_turnover: Option<f64>,
    avg_employees: Option<i64>,
}

impl PostV1ReportsSizeCategoryResponseCriteriaBuilder {
    pub fn total_assets(mut self, value: f64) -> Self {
        self.total_assets = Some(value);
        self
    }

    pub fn net_turnover(mut self, value: f64) -> Self {
        self.net_turnover = Some(value);
        self
    }

    pub fn avg_employees(mut self, value: i64) -> Self {
        self.avg_employees = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsSizeCategoryResponseCriteria`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_assets`](PostV1ReportsSizeCategoryResponseCriteriaBuilder::total_assets)
    /// - [`net_turnover`](PostV1ReportsSizeCategoryResponseCriteriaBuilder::net_turnover)
    /// - [`avg_employees`](PostV1ReportsSizeCategoryResponseCriteriaBuilder::avg_employees)
    pub fn build(self) -> Result<PostV1ReportsSizeCategoryResponseCriteria, BuildError> {
        Ok(PostV1ReportsSizeCategoryResponseCriteria {
            total_assets: self
                .total_assets
                .ok_or_else(|| BuildError::missing_field("total_assets"))?,
            net_turnover: self
                .net_turnover
                .ok_or_else(|| BuildError::missing_field("net_turnover"))?,
            avg_employees: self
                .avg_employees
                .ok_or_else(|| BuildError::missing_field("avg_employees"))?,
        })
    }
}
