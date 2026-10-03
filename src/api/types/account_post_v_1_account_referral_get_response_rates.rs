pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountReferralGetResponseRates {
    #[serde(rename = "perEur")]
    #[serde(default)]
    pub per_eur: i64,
    #[serde(rename = "pointCents")]
    #[serde(default)]
    pub point_cents: i64,
}

impl PostV1AccountReferralGetResponseRates {
    pub fn builder() -> PostV1AccountReferralGetResponseRatesBuilder {
        <PostV1AccountReferralGetResponseRatesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountReferralGetResponseRatesBuilder {
    per_eur: Option<i64>,
    point_cents: Option<i64>,
}

impl PostV1AccountReferralGetResponseRatesBuilder {
    pub fn per_eur(mut self, value: i64) -> Self {
        self.per_eur = Some(value);
        self
    }

    pub fn point_cents(mut self, value: i64) -> Self {
        self.point_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountReferralGetResponseRates`].
    /// This method will fail if any of the following fields are not set:
    /// - [`per_eur`](PostV1AccountReferralGetResponseRatesBuilder::per_eur)
    /// - [`point_cents`](PostV1AccountReferralGetResponseRatesBuilder::point_cents)
    pub fn build(self) -> Result<PostV1AccountReferralGetResponseRates, BuildError> {
        Ok(PostV1AccountReferralGetResponseRates {
            per_eur: self
                .per_eur
                .ok_or_else(|| BuildError::missing_field("per_eur"))?,
            point_cents: self
                .point_cents
                .ok_or_else(|| BuildError::missing_field("point_cents"))?,
        })
    }
}
