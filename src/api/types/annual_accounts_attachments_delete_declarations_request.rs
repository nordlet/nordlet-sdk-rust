pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsAttachmentsDeleteDeclarationsRequest {
    #[serde(default)]
    pub id: String,
}

impl AnnualAccountsAttachmentsDeleteDeclarationsRequest {
    pub fn builder() -> AnnualAccountsAttachmentsDeleteDeclarationsRequestBuilder {
        <AnnualAccountsAttachmentsDeleteDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsAttachmentsDeleteDeclarationsRequestBuilder {
    id: Option<String>,
}

impl AnnualAccountsAttachmentsDeleteDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsAttachmentsDeleteDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsAttachmentsDeleteDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<AnnualAccountsAttachmentsDeleteDeclarationsRequest, BuildError> {
        Ok(AnnualAccountsAttachmentsDeleteDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
