pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxProvisionPostLedgerRequest {
    #[serde(rename = "fiscalYear")]
    #[serde(default)]
    pub fiscal_year: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl TaxProvisionPostLedgerRequest {
    pub fn builder() -> TaxProvisionPostLedgerRequestBuilder {
        <TaxProvisionPostLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxProvisionPostLedgerRequestBuilder {
    fiscal_year: Option<i64>,
    amount: Option<String>,
}

impl TaxProvisionPostLedgerRequestBuilder {
    pub fn fiscal_year(mut self, value: i64) -> Self {
        self.fiscal_year = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxProvisionPostLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fiscal_year`](TaxProvisionPostLedgerRequestBuilder::fiscal_year)
    pub fn build(self) -> Result<TaxProvisionPostLedgerRequest, BuildError> {
        Ok(TaxProvisionPostLedgerRequest {
            fiscal_year: self
                .fiscal_year
                .ok_or_else(|| BuildError::missing_field("fiscal_year"))?,
            amount: self.amount,
        })
    }
}
