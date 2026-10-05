pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VehiclesCreateFleetResponseDocumentsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#ref: String,
}

impl VehiclesCreateFleetResponseDocumentsItem {
    pub fn builder() -> VehiclesCreateFleetResponseDocumentsItemBuilder {
        <VehiclesCreateFleetResponseDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VehiclesCreateFleetResponseDocumentsItemBuilder {
    name: Option<String>,
    r#ref: Option<String>,
}

impl VehiclesCreateFleetResponseDocumentsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VehiclesCreateFleetResponseDocumentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](VehiclesCreateFleetResponseDocumentsItemBuilder::name)
    /// - [`r#ref`](VehiclesCreateFleetResponseDocumentsItemBuilder::r#ref)
    pub fn build(self) -> Result<VehiclesCreateFleetResponseDocumentsItem, BuildError> {
        Ok(VehiclesCreateFleetResponseDocumentsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#ref: self
                .r#ref
                .ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
