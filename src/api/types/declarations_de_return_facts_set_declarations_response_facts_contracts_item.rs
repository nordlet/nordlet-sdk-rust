pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsResponseFactsContractsItem {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub partner: String,
    #[serde(default)]
    pub amount: String,
}

impl DeReturnFactsSetDeclarationsResponseFactsContractsItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsResponseFactsContractsItemBuilder {
        <DeReturnFactsSetDeclarationsResponseFactsContractsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsResponseFactsContractsItemBuilder {
    kind: Option<String>,
    date: Option<NaiveDate>,
    partner: Option<String>,
    amount: Option<String>,
}

impl DeReturnFactsSetDeclarationsResponseFactsContractsItemBuilder {
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

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsResponseFactsContractsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](DeReturnFactsSetDeclarationsResponseFactsContractsItemBuilder::kind)
    /// - [`date`](DeReturnFactsSetDeclarationsResponseFactsContractsItemBuilder::date)
    /// - [`partner`](DeReturnFactsSetDeclarationsResponseFactsContractsItemBuilder::partner)
    /// - [`amount`](DeReturnFactsSetDeclarationsResponseFactsContractsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsResponseFactsContractsItem, BuildError> {
        Ok(DeReturnFactsSetDeclarationsResponseFactsContractsItem {
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
