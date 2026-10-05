pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsDistributionsDeleteDeclarationsResponse {
    #[serde(default)]
    pub id: String,
}

impl AnnualAccountsDistributionsDeleteDeclarationsResponse {
    pub fn builder() -> AnnualAccountsDistributionsDeleteDeclarationsResponseBuilder {
        <AnnualAccountsDistributionsDeleteDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsDistributionsDeleteDeclarationsResponseBuilder {
    id: Option<String>,
}

impl AnnualAccountsDistributionsDeleteDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsDistributionsDeleteDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsDistributionsDeleteDeclarationsResponseBuilder::id)
    pub fn build(
        self,
    ) -> Result<AnnualAccountsDistributionsDeleteDeclarationsResponse, BuildError> {
        Ok(AnnualAccountsDistributionsDeleteDeclarationsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
