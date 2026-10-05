pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsSignaturesDeleteDeclarationsResponse {
    #[serde(default)]
    pub id: String,
}

impl AnnualAccountsSignaturesDeleteDeclarationsResponse {
    pub fn builder() -> AnnualAccountsSignaturesDeleteDeclarationsResponseBuilder {
        <AnnualAccountsSignaturesDeleteDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsSignaturesDeleteDeclarationsResponseBuilder {
    id: Option<String>,
}

impl AnnualAccountsSignaturesDeleteDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsSignaturesDeleteDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsSignaturesDeleteDeclarationsResponseBuilder::id)
    pub fn build(self) -> Result<AnnualAccountsSignaturesDeleteDeclarationsResponse, BuildError> {
        Ok(AnnualAccountsSignaturesDeleteDeclarationsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
