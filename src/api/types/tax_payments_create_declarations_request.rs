pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaxPaymentsCreateDeclarationsRequest {
    pub tax: TaxPaymentsCreateDeclarationsRequestTax,
    #[serde(default)]
    pub year: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    pub kind: TaxPaymentsCreateDeclarationsRequestKind,
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "paidOn")]
    #[serde(default)]
    pub paid_on: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default)]
    pub description: String,
}

impl TaxPaymentsCreateDeclarationsRequest {
    pub fn builder() -> TaxPaymentsCreateDeclarationsRequestBuilder {
        <TaxPaymentsCreateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxPaymentsCreateDeclarationsRequestBuilder {
    tax: Option<TaxPaymentsCreateDeclarationsRequestTax>,
    year: Option<i64>,
    month: Option<i64>,
    kind: Option<TaxPaymentsCreateDeclarationsRequestKind>,
    amount: Option<String>,
    paid_on: Option<NaiveDate>,
    reference: Option<String>,
    description: Option<String>,
}

impl TaxPaymentsCreateDeclarationsRequestBuilder {
    pub fn tax(mut self, value: TaxPaymentsCreateDeclarationsRequestTax) -> Self {
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

    pub fn kind(mut self, value: TaxPaymentsCreateDeclarationsRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn paid_on(mut self, value: NaiveDate) -> Self {
        self.paid_on = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxPaymentsCreateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tax`](TaxPaymentsCreateDeclarationsRequestBuilder::tax)
    /// - [`year`](TaxPaymentsCreateDeclarationsRequestBuilder::year)
    /// - [`kind`](TaxPaymentsCreateDeclarationsRequestBuilder::kind)
    /// - [`amount`](TaxPaymentsCreateDeclarationsRequestBuilder::amount)
    /// - [`paid_on`](TaxPaymentsCreateDeclarationsRequestBuilder::paid_on)
    /// - [`description`](TaxPaymentsCreateDeclarationsRequestBuilder::description)
    pub fn build(self) -> Result<TaxPaymentsCreateDeclarationsRequest, BuildError> {
        Ok(TaxPaymentsCreateDeclarationsRequest {
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self.month,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            paid_on: self
                .paid_on
                .ok_or_else(|| BuildError::missing_field("paid_on"))?,
            reference: self.reference,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
