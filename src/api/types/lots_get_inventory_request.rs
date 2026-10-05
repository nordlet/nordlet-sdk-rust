pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LotsGetInventoryRequest {
    #[serde(default)]
    pub id: String,
}

impl LotsGetInventoryRequest {
    pub fn builder() -> LotsGetInventoryRequestBuilder {
        <LotsGetInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LotsGetInventoryRequestBuilder {
    id: Option<String>,
}

impl LotsGetInventoryRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LotsGetInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LotsGetInventoryRequestBuilder::id)
    pub fn build(self) -> Result<LotsGetInventoryRequest, BuildError> {
        Ok(LotsGetInventoryRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
