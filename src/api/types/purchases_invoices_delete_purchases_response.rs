pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesDeletePurchasesResponse {
    #[serde(default)]
    pub id: String,
}

impl InvoicesDeletePurchasesResponse {
    pub fn builder() -> InvoicesDeletePurchasesResponseBuilder {
        <InvoicesDeletePurchasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesDeletePurchasesResponseBuilder {
    id: Option<String>,
}

impl InvoicesDeletePurchasesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesDeletePurchasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesDeletePurchasesResponseBuilder::id)
    pub fn build(self) -> Result<InvoicesDeletePurchasesResponse, BuildError> {
        Ok(InvoicesDeletePurchasesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
