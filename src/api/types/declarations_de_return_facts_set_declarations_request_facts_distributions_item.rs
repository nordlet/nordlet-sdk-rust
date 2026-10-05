pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsRequestFactsDistributionsItem {
    #[serde(rename = "resolutionDate")]
    #[serde(default)]
    pub resolution_date: NaiveDate,
    #[serde(rename = "paidOn")]
    #[serde(default)]
    pub paid_on: NaiveDate,
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "certifiedReduction")]
    #[serde(default)]
    pub certified_reduction: String,
}

impl DeReturnFactsSetDeclarationsRequestFactsDistributionsItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsRequestFactsDistributionsItemBuilder {
        <DeReturnFactsSetDeclarationsRequestFactsDistributionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsRequestFactsDistributionsItemBuilder {
    resolution_date: Option<NaiveDate>,
    paid_on: Option<NaiveDate>,
    amount: Option<String>,
    certified_reduction: Option<String>,
}

impl DeReturnFactsSetDeclarationsRequestFactsDistributionsItemBuilder {
    pub fn resolution_date(mut self, value: NaiveDate) -> Self {
        self.resolution_date = Some(value);
        self
    }

    pub fn paid_on(mut self, value: NaiveDate) -> Self {
        self.paid_on = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn certified_reduction(mut self, value: impl Into<String>) -> Self {
        self.certified_reduction = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsRequestFactsDistributionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resolution_date`](DeReturnFactsSetDeclarationsRequestFactsDistributionsItemBuilder::resolution_date)
    /// - [`paid_on`](DeReturnFactsSetDeclarationsRequestFactsDistributionsItemBuilder::paid_on)
    /// - [`amount`](DeReturnFactsSetDeclarationsRequestFactsDistributionsItemBuilder::amount)
    /// - [`certified_reduction`](DeReturnFactsSetDeclarationsRequestFactsDistributionsItemBuilder::certified_reduction)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsRequestFactsDistributionsItem, BuildError> {
        Ok(DeReturnFactsSetDeclarationsRequestFactsDistributionsItem {
            resolution_date: self
                .resolution_date
                .ok_or_else(|| BuildError::missing_field("resolution_date"))?,
            paid_on: self
                .paid_on
                .ok_or_else(|| BuildError::missing_field("paid_on"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            certified_reduction: self
                .certified_reduction
                .ok_or_else(|| BuildError::missing_field("certified_reduction"))?,
        })
    }
}
