pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsGetDeclarationsResponseFactsContributionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub date: NaiveDate,
    pub kind: DeReturnFactsGetDeclarationsResponseFactsContributionsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: String,
}

impl DeReturnFactsGetDeclarationsResponseFactsContributionsItem {
    pub fn builder() -> DeReturnFactsGetDeclarationsResponseFactsContributionsItemBuilder {
        <DeReturnFactsGetDeclarationsResponseFactsContributionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsGetDeclarationsResponseFactsContributionsItemBuilder {
    name: Option<String>,
    date: Option<NaiveDate>,
    kind: Option<DeReturnFactsGetDeclarationsResponseFactsContributionsItemKind>,
    description: Option<String>,
    amount: Option<String>,
}

impl DeReturnFactsGetDeclarationsResponseFactsContributionsItemBuilder {
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
        value: DeReturnFactsGetDeclarationsResponseFactsContributionsItemKind,
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

    /// Consumes the builder and constructs a [`DeReturnFactsGetDeclarationsResponseFactsContributionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DeReturnFactsGetDeclarationsResponseFactsContributionsItemBuilder::name)
    /// - [`date`](DeReturnFactsGetDeclarationsResponseFactsContributionsItemBuilder::date)
    /// - [`kind`](DeReturnFactsGetDeclarationsResponseFactsContributionsItemBuilder::kind)
    /// - [`amount`](DeReturnFactsGetDeclarationsResponseFactsContributionsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsGetDeclarationsResponseFactsContributionsItem, BuildError> {
        Ok(DeReturnFactsGetDeclarationsResponseFactsContributionsItem {
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
