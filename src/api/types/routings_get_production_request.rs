pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoutingsGetProductionRequest {
    #[serde(default)]
    pub id: String,
}

impl RoutingsGetProductionRequest {
    pub fn builder() -> RoutingsGetProductionRequestBuilder {
        <RoutingsGetProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoutingsGetProductionRequestBuilder {
    id: Option<String>,
}

impl RoutingsGetProductionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RoutingsGetProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RoutingsGetProductionRequestBuilder::id)
    pub fn build(self) -> Result<RoutingsGetProductionRequest, BuildError> {
        Ok(RoutingsGetProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
