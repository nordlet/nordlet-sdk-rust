pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsAttachmentsDeleteDeclarationsResponse {
    #[serde(default)]
    pub id: String,
}

impl AnnualAccountsAttachmentsDeleteDeclarationsResponse {
    pub fn builder() -> AnnualAccountsAttachmentsDeleteDeclarationsResponseBuilder {
        <AnnualAccountsAttachmentsDeleteDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsAttachmentsDeleteDeclarationsResponseBuilder {
    id: Option<String>,
}

impl AnnualAccountsAttachmentsDeleteDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsAttachmentsDeleteDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsAttachmentsDeleteDeclarationsResponseBuilder::id)
    pub fn build(self) -> Result<AnnualAccountsAttachmentsDeleteDeclarationsResponse, BuildError> {
        Ok(AnnualAccountsAttachmentsDeleteDeclarationsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
