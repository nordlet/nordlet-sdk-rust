pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandedCostsGetInventoryRequest {
    #[serde(default)]
    pub id: String,
}

impl LandedCostsGetInventoryRequest {
    pub fn builder() -> LandedCostsGetInventoryRequestBuilder {
        <LandedCostsGetInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandedCostsGetInventoryRequestBuilder {
    id: Option<String>,
}

impl LandedCostsGetInventoryRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LandedCostsGetInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LandedCostsGetInventoryRequestBuilder::id)
    pub fn build(self) -> Result<LandedCostsGetInventoryRequest, BuildError> {
        Ok(LandedCostsGetInventoryRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
