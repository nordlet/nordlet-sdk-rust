pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesDeleteSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesDeleteSalesRequest {
    pub fn builder() -> InvoicesDeleteSalesRequestBuilder {
        <InvoicesDeleteSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesDeleteSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesDeleteSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesDeleteSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesDeleteSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesDeleteSalesRequest, BuildError> {
        Ok(InvoicesDeleteSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
