pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsDistributionsUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "decidedOn")]
    #[serde(default)]
    pub decided_on: String,
    pub kind: PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsDistributionsUpdateResponse {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseBuilder {
        <PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseBuilder {
    id: Option<String>,
    decided_on: Option<String>,
    kind: Option<PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseKind>,
    amount: Option<String>,
    description: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseBuilder {
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
        value: PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseKind,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsDistributionsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseBuilder::id)
    /// - [`decided_on`](PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseBuilder::decided_on)
    /// - [`kind`](PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseBuilder::kind)
    /// - [`amount`](PostV1DeclarationsAnnualAccountsDistributionsUpdateResponseBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsDistributionsUpdateResponse, BuildError> {
        Ok(
            PostV1DeclarationsAnnualAccountsDistributionsUpdateResponse {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                decided_on: self
                    .decided_on
                    .ok_or_else(|| BuildError::missing_field("decided_on"))?,
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                amount: self
                    .amount
                    .ok_or_else(|| BuildError::missing_field("amount"))?,
                description: self.description,
            },
        )
    }
}
