pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesDeleteSalesResponse {
    #[serde(default)]
    pub id: String,
}

impl InvoicesDeleteSalesResponse {
    pub fn builder() -> InvoicesDeleteSalesResponseBuilder {
        <InvoicesDeleteSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesDeleteSalesResponseBuilder {
    id: Option<String>,
}

impl InvoicesDeleteSalesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesDeleteSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesDeleteSalesResponseBuilder::id)
    pub fn build(self) -> Result<InvoicesDeleteSalesResponse, BuildError> {
        Ok(InvoicesDeleteSalesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
