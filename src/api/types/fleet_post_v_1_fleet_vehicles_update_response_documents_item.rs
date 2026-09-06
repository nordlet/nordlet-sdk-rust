pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetVehiclesUpdateResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1FleetVehiclesUpdateResponseDocumentsItem {
    pub fn builder() -> PostV1FleetVehiclesUpdateResponseDocumentsItemBuilder {
        <PostV1FleetVehiclesUpdateResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesUpdateResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1FleetVehiclesUpdateResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesUpdateResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1FleetVehiclesUpdateResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1FleetVehiclesUpdateResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1FleetVehiclesUpdateResponseDocumentsItem, BuildError> {
        Ok(PostV1FleetVehiclesUpdateResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
