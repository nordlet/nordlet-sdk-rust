pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnnualAccountsDistributionsUpdateDeclarationsResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "decidedOn")]
    #[serde(default)]
    pub decided_on: String,
    pub kind: AnnualAccountsDistributionsUpdateDeclarationsResponseKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AnnualAccountsDistributionsUpdateDeclarationsResponse {
    pub fn builder() -> AnnualAccountsDistributionsUpdateDeclarationsResponseBuilder {
        <AnnualAccountsDistributionsUpdateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsDistributionsUpdateDeclarationsResponseBuilder {
    id: Option<String>,
    decided_on: Option<String>,
    kind: Option<AnnualAccountsDistributionsUpdateDeclarationsResponseKind>,
    amount: Option<String>,
    description: Option<String>,
}

impl AnnualAccountsDistributionsUpdateDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn decided_on(mut self, value: impl Into<String>) -> Self {
        self.decided_on = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: AnnualAccountsDistributionsUpdateDeclarationsResponseKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsDistributionsUpdateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsDistributionsUpdateDeclarationsResponseBuilder::id)
    /// - [`decided_on`](AnnualAccountsDistributionsUpdateDeclarationsResponseBuilder::decided_on)
    /// - [`kind`](AnnualAccountsDistributionsUpdateDeclarationsResponseBuilder::kind)
    /// - [`amount`](AnnualAccountsDistributionsUpdateDeclarationsResponseBuilder::amount)
    pub fn build(
        self,
    ) -> Result<AnnualAccountsDistributionsUpdateDeclarationsResponse, BuildError> {
        Ok(AnnualAccountsDistributionsUpdateDeclarationsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            decided_on: self
                .decided_on
                .ok_or_else(|| BuildError::missing_field("decided_on"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self.description,
        })
    }
}
