pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingAccountSetPlanResponseTopUp {
    #[serde(rename = "minCents")]
    #[serde(default)]
    pub min_cents: i64,
    #[serde(rename = "maxCents")]
    #[serde(default)]
    pub max_cents: i64,
}

impl PostV1BillingAccountSetPlanResponseTopUp {
    pub fn builder() -> PostV1BillingAccountSetPlanResponseTopUpBuilder {
        <PostV1BillingAccountSetPlanResponseTopUpBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingAccountSetPlanResponseTopUpBuilder {
    min_cents: Option<i64>,
    max_cents: Option<i64>,
}

impl PostV1BillingAccountSetPlanResponseTopUpBuilder {
    pub fn min_cents(mut self, value: i64) -> Self {
        self.min_cents = Some(value);
        self
    }

    pub fn max_cents(mut self, value: i64) -> Self {
        self.max_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingAccountSetPlanResponseTopUp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`min_cents`](PostV1BillingAccountSetPlanResponseTopUpBuilder::min_cents)
    /// - [`max_cents`](PostV1BillingAccountSetPlanResponseTopUpBuilder::max_cents)
    pub fn build(self) -> Result<PostV1BillingAccountSetPlanResponseTopUp, BuildError> {
        Ok(PostV1BillingAccountSetPlanResponseTopUp {
            min_cents: self
                .min_cents
                .ok_or_else(|| BuildError::missing_field("min_cents"))?,
            max_cents: self
                .max_cents
                .ok_or_else(|| BuildError::missing_field("max_cents"))?,
        })
    }
}
