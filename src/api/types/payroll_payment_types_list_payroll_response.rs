pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentTypesListPayrollResponse {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "defaultPaymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_type: Option<String>,
    #[serde(default)]
    pub rows: Vec<PaymentTypesListPayrollResponseRowsItem>,
}

impl PaymentTypesListPayrollResponse {
    pub fn builder() -> PaymentTypesListPayrollResponseBuilder {
        <PaymentTypesListPayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentTypesListPayrollResponseBuilder {
    country_code: Option<String>,
    default_payment_type: Option<String>,
    rows: Option<Vec<PaymentTypesListPayrollResponseRowsItem>>,
}

impl PaymentTypesListPayrollResponseBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn default_payment_type(mut self, value: impl Into<String>) -> Self {
        self.default_payment_type = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PaymentTypesListPayrollResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentTypesListPayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PaymentTypesListPayrollResponseBuilder::country_code)
    /// - [`rows`](PaymentTypesListPayrollResponseBuilder::rows)
    pub fn build(self) -> Result<PaymentTypesListPayrollResponse, BuildError> {
        Ok(PaymentTypesListPayrollResponse {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            default_payment_type: self.default_payment_type,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
