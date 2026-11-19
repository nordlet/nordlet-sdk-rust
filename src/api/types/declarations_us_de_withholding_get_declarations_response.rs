pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsDeWithholdingGetDeclarationsResponse {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub employees: f64,
    #[serde(default)]
    pub wages: String,
    #[serde(rename = "taxWithheld")]
    #[serde(default)]
    pub tax_withheld: String,
}

impl UsDeWithholdingGetDeclarationsResponse {
    pub fn builder() -> UsDeWithholdingGetDeclarationsResponseBuilder {
        <UsDeWithholdingGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeWithholdingGetDeclarationsResponseBuilder {
    from: Option<String>,
    to: Option<String>,
    employees: Option<f64>,
    wages: Option<String>,
    tax_withheld: Option<String>,
}

impl UsDeWithholdingGetDeclarationsResponseBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn employees(mut self, value: f64) -> Self {
        self.employees = Some(value);
        self
    }

    pub fn wages(mut self, value: impl Into<String>) -> Self {
        self.wages = Some(value.into());
        self
    }

    pub fn tax_withheld(mut self, value: impl Into<String>) -> Self {
        self.tax_withheld = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsDeWithholdingGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](UsDeWithholdingGetDeclarationsResponseBuilder::from)
    /// - [`to`](UsDeWithholdingGetDeclarationsResponseBuilder::to)
    /// - [`employees`](UsDeWithholdingGetDeclarationsResponseBuilder::employees)
    /// - [`wages`](UsDeWithholdingGetDeclarationsResponseBuilder::wages)
    /// - [`tax_withheld`](UsDeWithholdingGetDeclarationsResponseBuilder::tax_withheld)
    pub fn build(self) -> Result<UsDeWithholdingGetDeclarationsResponse, BuildError> {
        Ok(UsDeWithholdingGetDeclarationsResponse {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            employees: self
                .employees
                .ok_or_else(|| BuildError::missing_field("employees"))?,
            wages: self
                .wages
                .ok_or_else(|| BuildError::missing_field("wages"))?,
            tax_withheld: self
                .tax_withheld
                .ok_or_else(|| BuildError::missing_field("tax_withheld"))?,
        })
    }
}
