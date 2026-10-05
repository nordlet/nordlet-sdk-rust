pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsSignaturesDeleteDeclarationsRequest {
    #[serde(default)]
    pub id: String,
}

impl AnnualAccountsSignaturesDeleteDeclarationsRequest {
    pub fn builder() -> AnnualAccountsSignaturesDeleteDeclarationsRequestBuilder {
        <AnnualAccountsSignaturesDeleteDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsSignaturesDeleteDeclarationsRequestBuilder {
    id: Option<String>,
}

impl AnnualAccountsSignaturesDeleteDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsSignaturesDeleteDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsSignaturesDeleteDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<AnnualAccountsSignaturesDeleteDeclarationsRequest, BuildError> {
        Ok(AnnualAccountsSignaturesDeleteDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
