pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm313ComputeRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "payoutTiming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_timing: Option<PostV1DeclarationsLtGpm313ComputeRequestPayoutTiming>,
    #[serde(rename = "paymentDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_day: Option<i64>,
}

impl PostV1DeclarationsLtGpm313ComputeRequest {
    pub fn builder() -> PostV1DeclarationsLtGpm313ComputeRequestBuilder {
        <PostV1DeclarationsLtGpm313ComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm313ComputeRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    payout_timing: Option<PostV1DeclarationsLtGpm313ComputeRequestPayoutTiming>,
    payment_day: Option<i64>,
}

impl PostV1DeclarationsLtGpm313ComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn payout_timing(
        mut self,
        value: PostV1DeclarationsLtGpm313ComputeRequestPayoutTiming,
    ) -> Self {
        self.payout_timing = Some(value);
        self
    }

    pub fn payment_day(mut self, value: i64) -> Self {
        self.payment_day = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm313ComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtGpm313ComputeRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsLtGpm313ComputeRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm313ComputeRequest, BuildError> {
        Ok(PostV1DeclarationsLtGpm313ComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            payout_timing: self.payout_timing,
            payment_day: self.payment_day,
        })
    }
}
