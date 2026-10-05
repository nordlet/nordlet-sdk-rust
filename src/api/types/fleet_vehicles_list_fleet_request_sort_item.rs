pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VehiclesListFleetRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<VehiclesListFleetRequestSortItemDir>,
}

impl VehiclesListFleetRequestSortItem {
    pub fn builder() -> VehiclesListFleetRequestSortItemBuilder {
        <VehiclesListFleetRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VehiclesListFleetRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<VehiclesListFleetRequestSortItemDir>,
}

impl VehiclesListFleetRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: VehiclesListFleetRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VehiclesListFleetRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](VehiclesListFleetRequestSortItemBuilder::field)
    pub fn build(self) -> Result<VehiclesListFleetRequestSortItem, BuildError> {
        Ok(VehiclesListFleetRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
