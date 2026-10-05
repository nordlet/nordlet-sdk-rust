pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsGetDeclarationsResponseFactsContractsItem {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub partner: String,
    #[serde(default)]
    pub amount: String,
}

impl DeReturnFactsGetDeclarationsResponseFactsContractsItem {
    pub fn builder() -> DeReturnFactsGetDeclarationsResponseFactsContractsItemBuilder {
        <DeReturnFactsGetDeclarationsResponseFactsContractsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsGetDeclarationsResponseFactsContractsItemBuilder {
    kind: Option<String>,
    date: Option<NaiveDate>,
    partner: Option<String>,
    amount: Option<String>,
}

impl DeReturnFactsGetDeclarationsResponseFactsContractsItemBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn partner(mut self, value: impl Into<String>) -> Self {
        self.partner = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsGetDeclarationsResponseFactsContractsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](DeReturnFactsGetDeclarationsResponseFactsContractsItemBuilder::kind)
    /// - [`date`](DeReturnFactsGetDeclarationsResponseFactsContractsItemBuilder::date)
    /// - [`partner`](DeReturnFactsGetDeclarationsResponseFactsContractsItemBuilder::partner)
    /// - [`amount`](DeReturnFactsGetDeclarationsResponseFactsContractsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsGetDeclarationsResponseFactsContractsItem, BuildError> {
        Ok(DeReturnFactsGetDeclarationsResponseFactsContractsItem {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            partner: self
                .partner
                .ok_or_else(|| BuildError::missing_field("partner"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
