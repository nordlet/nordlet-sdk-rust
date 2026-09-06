pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetVehiclesGetResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1FleetVehiclesGetResponseDocumentsItem {
    pub fn builder() -> PostV1FleetVehiclesGetResponseDocumentsItemBuilder {
        <PostV1FleetVehiclesGetResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesGetResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1FleetVehiclesGetResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesGetResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1FleetVehiclesGetResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1FleetVehiclesGetResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1FleetVehiclesGetResponseDocumentsItem, BuildError> {
        Ok(PostV1FleetVehiclesGetResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
