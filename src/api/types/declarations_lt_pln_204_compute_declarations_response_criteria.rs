pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LtPln204ComputeDeclarationsResponseCriteria {
    #[serde(rename = "netTurnover")]
    #[serde(default)]
    pub net_turnover: String,
    #[serde(rename = "avgEmployees")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub avg_employees: f64,
}

impl LtPln204ComputeDeclarationsResponseCriteria {
    pub fn builder() -> LtPln204ComputeDeclarationsResponseCriteriaBuilder {
        <LtPln204ComputeDeclarationsResponseCriteriaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtPln204ComputeDeclarationsResponseCriteriaBuilder {
    net_turnover: Option<String>,
    avg_employees: Option<f64>,
}

impl LtPln204ComputeDeclarationsResponseCriteriaBuilder {
    pub fn net_turnover(mut self, value: impl Into<String>) -> Self {
        self.net_turnover = Some(value.into());
        self
    }

    pub fn avg_employees(mut self, value: f64) -> Self {
        self.avg_employees = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtPln204ComputeDeclarationsResponseCriteria`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net_turnover`](LtPln204ComputeDeclarationsResponseCriteriaBuilder::net_turnover)
    /// - [`avg_employees`](LtPln204ComputeDeclarationsResponseCriteriaBuilder::avg_employees)
    pub fn build(self) -> Result<LtPln204ComputeDeclarationsResponseCriteria, BuildError> {
        Ok(LtPln204ComputeDeclarationsResponseCriteria {
            net_turnover: self
                .net_turnover
                .ok_or_else(|| BuildError::missing_field("net_turnover"))?,
            avg_employees: self
                .avg_employees
                .ok_or_else(|| BuildError::missing_field("avg_employees"))?,
        })
    }
}
