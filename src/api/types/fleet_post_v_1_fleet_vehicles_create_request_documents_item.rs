pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetVehiclesCreateRequestDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1FleetVehiclesCreateRequestDocumentsItem {
    pub fn builder() -> PostV1FleetVehiclesCreateRequestDocumentsItemBuilder {
        <PostV1FleetVehiclesCreateRequestDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesCreateRequestDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1FleetVehiclesCreateRequestDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesCreateRequestDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1FleetVehiclesCreateRequestDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1FleetVehiclesCreateRequestDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1FleetVehiclesCreateRequestDocumentsItem, BuildError> {
        Ok(PostV1FleetVehiclesCreateRequestDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
