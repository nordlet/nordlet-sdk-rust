pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtGpm313ComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "payoutTiming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_timing: Option<LtGpm313ComputeDeclarationsRequestPayoutTiming>,
    #[serde(rename = "paymentDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_day: Option<i64>,
}

impl LtGpm313ComputeDeclarationsRequest {
    pub fn builder() -> LtGpm313ComputeDeclarationsRequestBuilder {
        <LtGpm313ComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtGpm313ComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    payout_timing: Option<LtGpm313ComputeDeclarationsRequestPayoutTiming>,
    payment_day: Option<i64>,
}

impl LtGpm313ComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn payout_timing(mut self, value: LtGpm313ComputeDeclarationsRequestPayoutTiming) -> Self {
        self.payout_timing = Some(value);
        self
    }

    pub fn payment_day(mut self, value: i64) -> Self {
        self.payment_day = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtGpm313ComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtGpm313ComputeDeclarationsRequestBuilder::year)
    /// - [`month`](LtGpm313ComputeDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<LtGpm313ComputeDeclarationsRequest, BuildError> {
        Ok(LtGpm313ComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            payout_timing: self.payout_timing,
            payment_day: self.payment_day,
        })
    }
}
