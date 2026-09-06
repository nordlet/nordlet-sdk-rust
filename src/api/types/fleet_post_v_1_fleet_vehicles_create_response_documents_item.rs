pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetVehiclesCreateResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1FleetVehiclesCreateResponseDocumentsItem {
    pub fn builder() -> PostV1FleetVehiclesCreateResponseDocumentsItemBuilder {
        <PostV1FleetVehiclesCreateResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesCreateResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1FleetVehiclesCreateResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesCreateResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1FleetVehiclesCreateResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1FleetVehiclesCreateResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1FleetVehiclesCreateResponseDocumentsItem, BuildError> {
        Ok(PostV1FleetVehiclesCreateResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
