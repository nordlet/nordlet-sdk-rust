pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferralGetAccountResponseRates {
    #[serde(rename = "perEur")]
    #[serde(default)]
    pub per_eur: i64,
    #[serde(rename = "pointCents")]
    #[serde(default)]
    pub point_cents: i64,
}

impl ReferralGetAccountResponseRates {
    pub fn builder() -> ReferralGetAccountResponseRatesBuilder {
        <ReferralGetAccountResponseRatesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferralGetAccountResponseRatesBuilder {
    per_eur: Option<i64>,
    point_cents: Option<i64>,
}

impl ReferralGetAccountResponseRatesBuilder {
    pub fn per_eur(mut self, value: i64) -> Self {
        self.per_eur = Some(value);
        self
    }

    pub fn point_cents(mut self, value: i64) -> Self {
        self.point_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferralGetAccountResponseRates`].
    /// This method will fail if any of the following fields are not set:
    /// - [`per_eur`](ReferralGetAccountResponseRatesBuilder::per_eur)
    /// - [`point_cents`](ReferralGetAccountResponseRatesBuilder::point_cents)
    pub fn build(self) -> Result<ReferralGetAccountResponseRates, BuildError> {
        Ok(ReferralGetAccountResponseRates {
            per_eur: self
                .per_eur
                .ok_or_else(|| BuildError::missing_field("per_eur"))?,
            point_cents: self
                .point_cents
                .ok_or_else(|| BuildError::missing_field("point_cents"))?,
        })
    }
}
