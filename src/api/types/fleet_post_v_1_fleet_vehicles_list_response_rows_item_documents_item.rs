pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetVehiclesListResponseRowsItemDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl PostV1FleetVehiclesListResponseRowsItemDocumentsItem {
    pub fn builder() -> PostV1FleetVehiclesListResponseRowsItemDocumentsItemBuilder {
        <PostV1FleetVehiclesListResponseRowsItemDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesListResponseRowsItemDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl PostV1FleetVehiclesListResponseRowsItemDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesListResponseRowsItemDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1FleetVehiclesListResponseRowsItemDocumentsItemBuilder::name)
    /// - [`r#ref`](PostV1FleetVehiclesListResponseRowsItemDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<PostV1FleetVehiclesListResponseRowsItemDocumentsItem, BuildError> {
        Ok(PostV1FleetVehiclesListResponseRowsItemDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
