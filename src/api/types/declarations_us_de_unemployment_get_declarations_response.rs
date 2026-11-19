pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsDeUnemploymentGetDeclarationsResponse {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub year: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quarter: f64,
    #[serde(rename = "totalWages")]
    #[serde(default)]
    pub total_wages: String,
    #[serde(rename = "excessWages")]
    #[serde(default)]
    pub excess_wages: String,
    #[serde(rename = "taxableWages")]
    #[serde(default)]
    pub taxable_wages: String,
    #[serde(default)]
    pub contributions: String,
    #[serde(default)]
    pub employees: Vec<UsDeUnemploymentGetDeclarationsResponseEmployeesItem>,
}

impl UsDeUnemploymentGetDeclarationsResponse {
    pub fn builder() -> UsDeUnemploymentGetDeclarationsResponseBuilder {
        <UsDeUnemploymentGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeUnemploymentGetDeclarationsResponseBuilder {
    year: Option<f64>,
    quarter: Option<f64>,
    total_wages: Option<String>,
    excess_wages: Option<String>,
    taxable_wages: Option<String>,
    contributions: Option<String>,
    employees: Option<Vec<UsDeUnemploymentGetDeclarationsResponseEmployeesItem>>,
}

impl UsDeUnemploymentGetDeclarationsResponseBuilder {
    pub fn year(mut self, value: f64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: f64) -> Self {
        self.quarter = Some(value);
        self
    }

    pub fn total_wages(mut self, value: impl Into<String>) -> Self {
        self.total_wages = Some(value.into());
        self
    }

    pub fn excess_wages(mut self, value: impl Into<String>) -> Self {
        self.excess_wages = Some(value.into());
        self
    }

    pub fn taxable_wages(mut self, value: impl Into<String>) -> Self {
        self.taxable_wages = Some(value.into());
        self
    }

    pub fn contributions(mut self, value: impl Into<String>) -> Self {
        self.contributions = Some(value.into());
        self
    }

    pub fn employees(
        mut self,
        value: Vec<UsDeUnemploymentGetDeclarationsResponseEmployeesItem>,
    ) -> Self {
        self.employees = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsDeUnemploymentGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](UsDeUnemploymentGetDeclarationsResponseBuilder::year)
    /// - [`quarter`](UsDeUnemploymentGetDeclarationsResponseBuilder::quarter)
    /// - [`total_wages`](UsDeUnemploymentGetDeclarationsResponseBuilder::total_wages)
    /// - [`excess_wages`](UsDeUnemploymentGetDeclarationsResponseBuilder::excess_wages)
    /// - [`taxable_wages`](UsDeUnemploymentGetDeclarationsResponseBuilder::taxable_wages)
    /// - [`contributions`](UsDeUnemploymentGetDeclarationsResponseBuilder::contributions)
    /// - [`employees`](UsDeUnemploymentGetDeclarationsResponseBuilder::employees)
    pub fn build(self) -> Result<UsDeUnemploymentGetDeclarationsResponse, BuildError> {
        Ok(UsDeUnemploymentGetDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
            total_wages: self
                .total_wages
                .ok_or_else(|| BuildError::missing_field("total_wages"))?,
            excess_wages: self
                .excess_wages
                .ok_or_else(|| BuildError::missing_field("excess_wages"))?,
            taxable_wages: self
                .taxable_wages
                .ok_or_else(|| BuildError::missing_field("taxable_wages"))?,
            contributions: self
                .contributions
                .ok_or_else(|| BuildError::missing_field("contributions"))?,
            employees: self
                .employees
                .ok_or_else(|| BuildError::missing_field("employees"))?,
        })
    }
}
