pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItem {
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

impl PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItemBuilder {
        <PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItemBuilder {
    resolution_date: Option<String>,
    paid_on: Option<String>,
    amount: Option<String>,
    certified_reduction: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resolution_date`](PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItemBuilder::resolution_date)
    /// - [`paid_on`](PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItemBuilder::paid_on)
    /// - [`amount`](PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItemBuilder::amount)
    /// - [`certified_reduction`](PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItemBuilder::certified_reduction)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsGetResponseFactsDistributionsItem {
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
