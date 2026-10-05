pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesGetSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesGetSalesRequest {
    pub fn builder() -> InvoicesGetSalesRequestBuilder {
        <InvoicesGetSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesGetSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesGetSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesGetSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesGetSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesGetSalesRequest, BuildError> {
        Ok(InvoicesGetSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
