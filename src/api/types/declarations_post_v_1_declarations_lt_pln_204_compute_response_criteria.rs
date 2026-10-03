pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1DeclarationsLtPln204ComputeResponseCriteria {
    #[serde(rename = "netTurnover")]
    #[serde(default)]
    pub net_turnover: String,
    #[serde(rename = "avgEmployees")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub avg_employees: f64,
}

impl PostV1DeclarationsLtPln204ComputeResponseCriteria {
    pub fn builder() -> PostV1DeclarationsLtPln204ComputeResponseCriteriaBuilder {
        <PostV1DeclarationsLtPln204ComputeResponseCriteriaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtPln204ComputeResponseCriteriaBuilder {
    net_turnover: Option<String>,
    avg_employees: Option<f64>,
}

impl PostV1DeclarationsLtPln204ComputeResponseCriteriaBuilder {
    pub fn net_turnover(mut self, value: impl Into<String>) -> Self {
        self.net_turnover = Some(value.into());
        self
    }

    pub fn avg_employees(mut self, value: f64) -> Self {
        self.avg_employees = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtPln204ComputeResponseCriteria`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net_turnover`](PostV1DeclarationsLtPln204ComputeResponseCriteriaBuilder::net_turnover)
    /// - [`avg_employees`](PostV1DeclarationsLtPln204ComputeResponseCriteriaBuilder::avg_employees)
    pub fn build(self) -> Result<PostV1DeclarationsLtPln204ComputeResponseCriteria, BuildError> {
        Ok(PostV1DeclarationsLtPln204ComputeResponseCriteria {
            net_turnover: self
                .net_turnover
                .ok_or_else(|| BuildError::missing_field("net_turnover"))?,
            avg_employees: self
                .avg_employees
                .ok_or_else(|| BuildError::missing_field("avg_employees"))?,
        })
    }
}
