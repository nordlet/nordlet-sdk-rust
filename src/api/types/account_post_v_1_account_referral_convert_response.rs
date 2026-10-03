pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountReferralConvertResponse {
    #[serde(default)]
    pub points: i64,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(rename = "pointsLeft")]
    #[serde(default)]
    pub points_left: i64,
    #[serde(rename = "balanceCents")]
    #[serde(default)]
    pub balance_cents: i64,
}

impl PostV1AccountReferralConvertResponse {
    pub fn builder() -> PostV1AccountReferralConvertResponseBuilder {
        <PostV1AccountReferralConvertResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountReferralConvertResponseBuilder {
    points: Option<i64>,
    amount_cents: Option<i64>,
    points_left: Option<i64>,
    balance_cents: Option<i64>,
}

impl PostV1AccountReferralConvertResponseBuilder {
    pub fn points(mut self, value: i64) -> Self {
        self.points = Some(value);
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn points_left(mut self, value: i64) -> Self {
        self.points_left = Some(value);
        self
    }

    pub fn balance_cents(mut self, value: i64) -> Self {
        self.balance_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountReferralConvertResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`points`](PostV1AccountReferralConvertResponseBuilder::points)
    /// - [`amount_cents`](PostV1AccountReferralConvertResponseBuilder::amount_cents)
    /// - [`points_left`](PostV1AccountReferralConvertResponseBuilder::points_left)
    /// - [`balance_cents`](PostV1AccountReferralConvertResponseBuilder::balance_cents)
    pub fn build(self) -> Result<PostV1AccountReferralConvertResponse, BuildError> {
        Ok(PostV1AccountReferralConvertResponse {
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            points_left: self
                .points_left
                .ok_or_else(|| BuildError::missing_field("points_left"))?,
            balance_cents: self
                .balance_cents
                .ok_or_else(|| BuildError::missing_field("balance_cents"))?,
        })
    }
}
