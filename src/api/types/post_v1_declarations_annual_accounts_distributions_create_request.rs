pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsDistributionsCreateRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "decidedOn")]
    #[serde(default)]
    pub decided_on: String,
    pub kind: PostV1DeclarationsAnnualAccountsDistributionsCreateRequestKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsDistributionsCreateRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsDistributionsCreateRequestBuilder {
        <PostV1DeclarationsAnnualAccountsDistributionsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsDistributionsCreateRequestBuilder {
    year: Option<i64>,
    decided_on: Option<String>,
    kind: Option<PostV1DeclarationsAnnualAccountsDistributionsCreateRequestKind>,
    amount: Option<String>,
    description: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsDistributionsCreateRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn decided_on(mut self, value: impl Into<String>) -> Self {
        self.decided_on = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: PostV1DeclarationsAnnualAccountsDistributionsCreateRequestKind,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsDistributionsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsAnnualAccountsDistributionsCreateRequestBuilder::year)
    /// - [`decided_on`](PostV1DeclarationsAnnualAccountsDistributionsCreateRequestBuilder::decided_on)
    /// - [`kind`](PostV1DeclarationsAnnualAccountsDistributionsCreateRequestBuilder::kind)
    /// - [`amount`](PostV1DeclarationsAnnualAccountsDistributionsCreateRequestBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsDistributionsCreateRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsDistributionsCreateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
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
