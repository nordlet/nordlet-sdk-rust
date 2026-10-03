pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItem {
    #[serde(rename = "resolutionDate")]
    #[serde(default)]
    pub resolution_date: String,
    #[serde(rename = "paidOn")]
    #[serde(default)]
    pub paid_on: String,
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "certifiedReduction")]
    #[serde(default)]
    pub certified_reduction: String,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItemBuilder {
        <PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItemBuilder {
    resolution_date: Option<String>,
    paid_on: Option<String>,
    amount: Option<String>,
    certified_reduction: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItemBuilder {
    pub fn resolution_date(mut self, value: impl Into<String>) -> Self {
        self.resolution_date = Some(value.into());
        self
    }

    pub fn paid_on(mut self, value: impl Into<String>) -> Self {
        self.paid_on = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resolution_date`](PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItemBuilder::resolution_date)
    /// - [`paid_on`](PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItemBuilder::paid_on)
    /// - [`amount`](PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItemBuilder::amount)
    /// - [`certified_reduction`](PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItemBuilder::certified_reduction)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsSetRequestFactsDistributionsItem {
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
            },
        )
    }
}
