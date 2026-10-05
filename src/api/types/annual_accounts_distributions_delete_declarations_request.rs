pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsDistributionsDeleteDeclarationsRequest {
    #[serde(default)]
    pub id: String,
}

impl AnnualAccountsDistributionsDeleteDeclarationsRequest {
    pub fn builder() -> AnnualAccountsDistributionsDeleteDeclarationsRequestBuilder {
        <AnnualAccountsDistributionsDeleteDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsDistributionsDeleteDeclarationsRequestBuilder {
    id: Option<String>,
}

impl AnnualAccountsDistributionsDeleteDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsDistributionsDeleteDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsDistributionsDeleteDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<AnnualAccountsDistributionsDeleteDeclarationsRequest, BuildError> {
        Ok(AnnualAccountsDistributionsDeleteDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
