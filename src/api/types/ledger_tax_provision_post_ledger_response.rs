pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxProvisionPostLedgerResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "fiscalYear")]
    #[serde(default)]
    pub fiscal_year: i64,
    #[serde(default)]
    pub amount: String,
}

impl TaxProvisionPostLedgerResponse {
    pub fn builder() -> TaxProvisionPostLedgerResponseBuilder {
        <TaxProvisionPostLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxProvisionPostLedgerResponseBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    fiscal_year: Option<i64>,
    amount: Option<String>,
}

impl TaxProvisionPostLedgerResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn fiscal_year(mut self, value: i64) -> Self {
        self.fiscal_year = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxProvisionPostLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TaxProvisionPostLedgerResponseBuilder::id)
    /// - [`date`](TaxProvisionPostLedgerResponseBuilder::date)
    /// - [`fiscal_year`](TaxProvisionPostLedgerResponseBuilder::fiscal_year)
    /// - [`amount`](TaxProvisionPostLedgerResponseBuilder::amount)
    pub fn build(self) -> Result<TaxProvisionPostLedgerResponse, BuildError> {
        Ok(TaxProvisionPostLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            fiscal_year: self
                .fiscal_year
                .ok_or_else(|| BuildError::missing_field("fiscal_year"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
