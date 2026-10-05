pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VehiclesGetFleetRequest {
    #[serde(default)]
    pub id: String,
}

impl VehiclesGetFleetRequest {
    pub fn builder() -> VehiclesGetFleetRequestBuilder {
        <VehiclesGetFleetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VehiclesGetFleetRequestBuilder {
    id: Option<String>,
}

impl VehiclesGetFleetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VehiclesGetFleetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](VehiclesGetFleetRequestBuilder::id)
    pub fn build(self) -> Result<VehiclesGetFleetRequest, BuildError> {
        Ok(VehiclesGetFleetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
