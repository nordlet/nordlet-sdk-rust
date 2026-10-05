pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsGetPurchasesRequest {
    #[serde(default)]
    pub id: String,
}

impl ReceiptsGetPurchasesRequest {
    pub fn builder() -> ReceiptsGetPurchasesRequestBuilder {
        <ReceiptsGetPurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsGetPurchasesRequestBuilder {
    id: Option<String>,
}

impl ReceiptsGetPurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsGetPurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReceiptsGetPurchasesRequestBuilder::id)
    pub fn build(self) -> Result<ReceiptsGetPurchasesRequest, BuildError> {
        Ok(ReceiptsGetPurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
