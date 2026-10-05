pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsResponseFactsContributionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub date: NaiveDate,
    pub kind: DeReturnFactsSetDeclarationsResponseFactsContributionsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: String,
}

impl DeReturnFactsSetDeclarationsResponseFactsContributionsItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsResponseFactsContributionsItemBuilder {
        <DeReturnFactsSetDeclarationsResponseFactsContributionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsResponseFactsContributionsItemBuilder {
    name: Option<String>,
    date: Option<NaiveDate>,
    kind: Option<DeReturnFactsSetDeclarationsResponseFactsContributionsItemKind>,
    description: Option<String>,
    amount: Option<String>,
}

impl DeReturnFactsSetDeclarationsResponseFactsContributionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn kind(
        mut self,
        value: DeReturnFactsSetDeclarationsResponseFactsContributionsItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsResponseFactsContributionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DeReturnFactsSetDeclarationsResponseFactsContributionsItemBuilder::name)
    /// - [`date`](DeReturnFactsSetDeclarationsResponseFactsContributionsItemBuilder::date)
    /// - [`kind`](DeReturnFactsSetDeclarationsResponseFactsContributionsItemBuilder::kind)
    /// - [`amount`](DeReturnFactsSetDeclarationsResponseFactsContributionsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsResponseFactsContributionsItem, BuildError> {
        Ok(DeReturnFactsSetDeclarationsResponseFactsContributionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            description: self.description,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
