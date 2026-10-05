pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnnualAccountsDistributionsCreateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "decidedOn")]
    #[serde(default)]
    pub decided_on: NaiveDate,
    pub kind: AnnualAccountsDistributionsCreateDeclarationsRequestKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AnnualAccountsDistributionsCreateDeclarationsRequest {
    pub fn builder() -> AnnualAccountsDistributionsCreateDeclarationsRequestBuilder {
        <AnnualAccountsDistributionsCreateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsDistributionsCreateDeclarationsRequestBuilder {
    year: Option<i64>,
    decided_on: Option<NaiveDate>,
    kind: Option<AnnualAccountsDistributionsCreateDeclarationsRequestKind>,
    amount: Option<String>,
    description: Option<String>,
}

impl AnnualAccountsDistributionsCreateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn decided_on(mut self, value: NaiveDate) -> Self {
        self.decided_on = Some(value);
        self
    }

    pub fn kind(mut self, value: AnnualAccountsDistributionsCreateDeclarationsRequestKind) -> Self {
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

    /// Consumes the builder and constructs a [`AnnualAccountsDistributionsCreateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](AnnualAccountsDistributionsCreateDeclarationsRequestBuilder::year)
    /// - [`decided_on`](AnnualAccountsDistributionsCreateDeclarationsRequestBuilder::decided_on)
    /// - [`kind`](AnnualAccountsDistributionsCreateDeclarationsRequestBuilder::kind)
    /// - [`amount`](AnnualAccountsDistributionsCreateDeclarationsRequestBuilder::amount)
    pub fn build(self) -> Result<AnnualAccountsDistributionsCreateDeclarationsRequest, BuildError> {
        Ok(AnnualAccountsDistributionsCreateDeclarationsRequest {
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
