pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtGpm312ComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "payoutTiming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_timing: Option<LtGpm312ComputeDeclarationsRequestPayoutTiming>,
}

impl LtGpm312ComputeDeclarationsRequest {
    pub fn builder() -> LtGpm312ComputeDeclarationsRequestBuilder {
        <LtGpm312ComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtGpm312ComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    payout_timing: Option<LtGpm312ComputeDeclarationsRequestPayoutTiming>,
}

impl LtGpm312ComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn payout_timing(mut self, value: LtGpm312ComputeDeclarationsRequestPayoutTiming) -> Self {
        self.payout_timing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtGpm312ComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtGpm312ComputeDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<LtGpm312ComputeDeclarationsRequest, BuildError> {
        Ok(LtGpm312ComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            payout_timing: self.payout_timing,
        })
    }
}
