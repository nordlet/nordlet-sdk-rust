pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsDeFranchiseTaxComputeDeclarationsResponse {
    #[serde(rename = "authorizedShares")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub authorized_shares: f64,
    #[serde(rename = "authorizedSharesMethodTax")]
    #[serde(default)]
    pub authorized_shares_method_tax: String,
    #[serde(rename = "assumedPar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_par: Option<String>,
    #[serde(rename = "assumedParValueCapital")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_par_value_capital: Option<String>,
    #[serde(rename = "assumedNoParCapital")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_no_par_capital: Option<String>,
    #[serde(rename = "assumedParValueCapitalMethodTax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_par_value_capital_method_tax: Option<String>,
    pub method: UsDeFranchiseTaxComputeDeclarationsResponseMethod,
    #[serde(default)]
    pub tax: String,
    #[serde(rename = "annualReportFee")]
    #[serde(default)]
    pub annual_report_fee: String,
}

impl UsDeFranchiseTaxComputeDeclarationsResponse {
    pub fn builder() -> UsDeFranchiseTaxComputeDeclarationsResponseBuilder {
        <UsDeFranchiseTaxComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeFranchiseTaxComputeDeclarationsResponseBuilder {
    authorized_shares: Option<f64>,
    authorized_shares_method_tax: Option<String>,
    assumed_par: Option<String>,
    assumed_par_value_capital: Option<String>,
    assumed_no_par_capital: Option<String>,
    assumed_par_value_capital_method_tax: Option<String>,
    method: Option<UsDeFranchiseTaxComputeDeclarationsResponseMethod>,
    tax: Option<String>,
    annual_report_fee: Option<String>,
}

impl UsDeFranchiseTaxComputeDeclarationsResponseBuilder {
    pub fn authorized_shares(mut self, value: f64) -> Self {
        self.authorized_shares = Some(value);
        self
    }

    pub fn authorized_shares_method_tax(mut self, value: impl Into<String>) -> Self {
        self.authorized_shares_method_tax = Some(value.into());
        self
    }

    pub fn assumed_par(mut self, value: impl Into<String>) -> Self {
        self.assumed_par = Some(value.into());
        self
    }

    pub fn assumed_par_value_capital(mut self, value: impl Into<String>) -> Self {
        self.assumed_par_value_capital = Some(value.into());
        self
    }

    pub fn assumed_no_par_capital(mut self, value: impl Into<String>) -> Self {
        self.assumed_no_par_capital = Some(value.into());
        self
    }

    pub fn assumed_par_value_capital_method_tax(mut self, value: impl Into<String>) -> Self {
        self.assumed_par_value_capital_method_tax = Some(value.into());
        self
    }

    pub fn method(mut self, value: UsDeFranchiseTaxComputeDeclarationsResponseMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn tax(mut self, value: impl Into<String>) -> Self {
        self.tax = Some(value.into());
        self
    }

    pub fn annual_report_fee(mut self, value: impl Into<String>) -> Self {
        self.annual_report_fee = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsDeFranchiseTaxComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authorized_shares`](UsDeFranchiseTaxComputeDeclarationsResponseBuilder::authorized_shares)
    /// - [`authorized_shares_method_tax`](UsDeFranchiseTaxComputeDeclarationsResponseBuilder::authorized_shares_method_tax)
    /// - [`method`](UsDeFranchiseTaxComputeDeclarationsResponseBuilder::method)
    /// - [`tax`](UsDeFranchiseTaxComputeDeclarationsResponseBuilder::tax)
    /// - [`annual_report_fee`](UsDeFranchiseTaxComputeDeclarationsResponseBuilder::annual_report_fee)
    pub fn build(self) -> Result<UsDeFranchiseTaxComputeDeclarationsResponse, BuildError> {
        Ok(UsDeFranchiseTaxComputeDeclarationsResponse {
            authorized_shares: self
                .authorized_shares
                .ok_or_else(|| BuildError::missing_field("authorized_shares"))?,
            authorized_shares_method_tax: self
                .authorized_shares_method_tax
                .ok_or_else(|| BuildError::missing_field("authorized_shares_method_tax"))?,
            assumed_par: self.assumed_par,
            assumed_par_value_capital: self.assumed_par_value_capital,
            assumed_no_par_capital: self.assumed_no_par_capital,
            assumed_par_value_capital_method_tax: self.assumed_par_value_capital_method_tax,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            annual_report_fee: self
                .annual_report_fee
                .ok_or_else(|| BuildError::missing_field("annual_report_fee"))?,
        })
    }
}
