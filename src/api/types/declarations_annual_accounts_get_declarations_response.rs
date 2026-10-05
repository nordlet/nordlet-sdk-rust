pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsGetDeclarationsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval: Option<AnnualAccountsGetDeclarationsResponseApproval>,
}

impl AnnualAccountsGetDeclarationsResponse {
    pub fn builder() -> AnnualAccountsGetDeclarationsResponseBuilder {
        <AnnualAccountsGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsGetDeclarationsResponseBuilder {
    approval: Option<AnnualAccountsGetDeclarationsResponseApproval>,
}

impl AnnualAccountsGetDeclarationsResponseBuilder {
    pub fn approval(mut self, value: AnnualAccountsGetDeclarationsResponseApproval) -> Self {
        self.approval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsGetDeclarationsResponse`].
    pub fn build(self) -> Result<AnnualAccountsGetDeclarationsResponse, BuildError> {
        Ok(AnnualAccountsGetDeclarationsResponse {
            approval: self.approval,
        })
    }
}
