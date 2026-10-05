pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnnualAccountsDistributionsUpdateDeclarationsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "decidedOn")]
    #[serde(default)]
    pub decided_on: NaiveDate,
    pub kind: AnnualAccountsDistributionsUpdateDeclarationsRequestKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AnnualAccountsDistributionsUpdateDeclarationsRequest {
    pub fn builder() -> AnnualAccountsDistributionsUpdateDeclarationsRequestBuilder {
        <AnnualAccountsDistributionsUpdateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsDistributionsUpdateDeclarationsRequestBuilder {
    id: Option<String>,
    decided_on: Option<NaiveDate>,
    kind: Option<AnnualAccountsDistributionsUpdateDeclarationsRequestKind>,
    amount: Option<String>,
    description: Option<String>,
}

impl AnnualAccountsDistributionsUpdateDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn decided_on(mut self, value: NaiveDate) -> Self {
        self.decided_on = Some(value);
        self
    }

    pub fn kind(mut self, value: AnnualAccountsDistributionsUpdateDeclarationsRequestKind) -> Self {
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

    /// Consumes the builder and constructs a [`AnnualAccountsDistributionsUpdateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsDistributionsUpdateDeclarationsRequestBuilder::id)
    /// - [`decided_on`](AnnualAccountsDistributionsUpdateDeclarationsRequestBuilder::decided_on)
    /// - [`kind`](AnnualAccountsDistributionsUpdateDeclarationsRequestBuilder::kind)
    /// - [`amount`](AnnualAccountsDistributionsUpdateDeclarationsRequestBuilder::amount)
    pub fn build(self) -> Result<AnnualAccountsDistributionsUpdateDeclarationsRequest, BuildError> {
        Ok(AnnualAccountsDistributionsUpdateDeclarationsRequest {
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
