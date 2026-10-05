pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsRequestFactsContributionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub date: NaiveDate,
    pub kind: DeReturnFactsSetDeclarationsRequestFactsContributionsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: String,
}

impl DeReturnFactsSetDeclarationsRequestFactsContributionsItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsRequestFactsContributionsItemBuilder {
        <DeReturnFactsSetDeclarationsRequestFactsContributionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsRequestFactsContributionsItemBuilder {
    name: Option<String>,
    date: Option<NaiveDate>,
    kind: Option<DeReturnFactsSetDeclarationsRequestFactsContributionsItemKind>,
    description: Option<String>,
    amount: Option<String>,
}

impl DeReturnFactsSetDeclarationsRequestFactsContributionsItemBuilder {
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
        value: DeReturnFactsSetDeclarationsRequestFactsContributionsItemKind,
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

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsRequestFactsContributionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DeReturnFactsSetDeclarationsRequestFactsContributionsItemBuilder::name)
    /// - [`date`](DeReturnFactsSetDeclarationsRequestFactsContributionsItemBuilder::date)
    /// - [`kind`](DeReturnFactsSetDeclarationsRequestFactsContributionsItemBuilder::kind)
    /// - [`amount`](DeReturnFactsSetDeclarationsRequestFactsContributionsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsRequestFactsContributionsItem, BuildError> {
        Ok(DeReturnFactsSetDeclarationsRequestFactsContributionsItem {
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
