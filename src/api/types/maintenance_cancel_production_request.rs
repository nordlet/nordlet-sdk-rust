pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MaintenanceCancelProductionRequest {
    #[serde(default)]
    pub id: String,
}

impl MaintenanceCancelProductionRequest {
    pub fn builder() -> MaintenanceCancelProductionRequestBuilder {
        <MaintenanceCancelProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MaintenanceCancelProductionRequestBuilder {
    id: Option<String>,
}

impl MaintenanceCancelProductionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MaintenanceCancelProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MaintenanceCancelProductionRequestBuilder::id)
    pub fn build(self) -> Result<MaintenanceCancelProductionRequest, BuildError> {
        Ok(MaintenanceCancelProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
