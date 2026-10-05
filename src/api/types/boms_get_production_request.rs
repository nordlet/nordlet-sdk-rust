pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BomsGetProductionRequest {
    #[serde(default)]
    pub id: String,
}

impl BomsGetProductionRequest {
    pub fn builder() -> BomsGetProductionRequestBuilder {
        <BomsGetProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BomsGetProductionRequestBuilder {
    id: Option<String>,
}

impl BomsGetProductionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BomsGetProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BomsGetProductionRequestBuilder::id)
    pub fn build(self) -> Result<BomsGetProductionRequest, BuildError> {
        Ok(BomsGetProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
