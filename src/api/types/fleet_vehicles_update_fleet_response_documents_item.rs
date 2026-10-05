pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VehiclesUpdateFleetResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl VehiclesUpdateFleetResponseDocumentsItem {
    pub fn builder() -> VehiclesUpdateFleetResponseDocumentsItemBuilder {
        <VehiclesUpdateFleetResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VehiclesUpdateFleetResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl VehiclesUpdateFleetResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VehiclesUpdateFleetResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](VehiclesUpdateFleetResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](VehiclesUpdateFleetResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<VehiclesUpdateFleetResponseDocumentsItem, BuildError> {
        Ok(VehiclesUpdateFleetResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
