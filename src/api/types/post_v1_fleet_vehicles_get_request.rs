pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetVehiclesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1FleetVehiclesGetRequest {
    pub fn builder() -> PostV1FleetVehiclesGetRequestBuilder {
        <PostV1FleetVehiclesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1FleetVehiclesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1FleetVehiclesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1FleetVehiclesGetRequest, BuildError> {
        Ok(PostV1FleetVehiclesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
