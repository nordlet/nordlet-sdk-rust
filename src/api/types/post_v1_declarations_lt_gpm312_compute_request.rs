pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm312ComputeRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "payoutTiming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_timing: Option<PostV1DeclarationsLtGpm312ComputeRequestPayoutTiming>,
}

impl PostV1DeclarationsLtGpm312ComputeRequest {
    pub fn builder() -> PostV1DeclarationsLtGpm312ComputeRequestBuilder {
        <PostV1DeclarationsLtGpm312ComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm312ComputeRequestBuilder {
    year: Option<i64>,
    payout_timing: Option<PostV1DeclarationsLtGpm312ComputeRequestPayoutTiming>,
}

impl PostV1DeclarationsLtGpm312ComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn payout_timing(
        mut self,
        value: PostV1DeclarationsLtGpm312ComputeRequestPayoutTiming,
    ) -> Self {
        self.payout_timing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm312ComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtGpm312ComputeRequestBuilder::year)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm312ComputeRequest, BuildError> {
        Ok(PostV1DeclarationsLtGpm312ComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            payout_timing: self.payout_timing,
        })
    }
}
