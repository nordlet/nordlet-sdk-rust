pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaxPaymentsListDeclarationsRequest {
    pub tax: TaxPaymentsListDeclarationsRequestTax,
    #[serde(default)]
    pub year: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
}

impl TaxPaymentsListDeclarationsRequest {
    pub fn builder() -> TaxPaymentsListDeclarationsRequestBuilder {
        <TaxPaymentsListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxPaymentsListDeclarationsRequestBuilder {
    tax: Option<TaxPaymentsListDeclarationsRequestTax>,
    year: Option<i64>,
    month: Option<i64>,
}

impl TaxPaymentsListDeclarationsRequestBuilder {
    pub fn tax(mut self, value: TaxPaymentsListDeclarationsRequestTax) -> Self {
        self.tax = Some(value);
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaxPaymentsListDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tax`](TaxPaymentsListDeclarationsRequestBuilder::tax)
    /// - [`year`](TaxPaymentsListDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<TaxPaymentsListDeclarationsRequest, BuildError> {
        Ok(TaxPaymentsListDeclarationsRequest {
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self.month,
        })
    }
}
