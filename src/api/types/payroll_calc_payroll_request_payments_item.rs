pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalcPayrollRequestPaymentsItem {
    #[serde(rename = "paymentType")]
    #[serde(default)]
    pub payment_type: String,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
}

impl CalcPayrollRequestPaymentsItem {
    pub fn builder() -> CalcPayrollRequestPaymentsItemBuilder {
        <CalcPayrollRequestPaymentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalcPayrollRequestPaymentsItemBuilder {
    payment_type: Option<String>,
    amount: Option<String>,
    hours: Option<String>,
}

impl CalcPayrollRequestPaymentsItemBuilder {
    pub fn payment_type(mut self, value: impl Into<String>) -> Self {
        self.payment_type = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CalcPayrollRequestPaymentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_type`](CalcPayrollRequestPaymentsItemBuilder::payment_type)
    /// - [`amount`](CalcPayrollRequestPaymentsItemBuilder::amount)
    pub fn build(self) -> Result<CalcPayrollRequestPaymentsItem, BuildError> {
        Ok(CalcPayrollRequestPaymentsItem {
            payment_type: self
                .payment_type
                .ok_or_else(|| BuildError::missing_field("payment_type"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            hours: self.hours,
        })
    }
}
