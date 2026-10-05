pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MaintenanceListProductionRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<MaintenanceListProductionRequestSortItemDir>,
}

impl MaintenanceListProductionRequestSortItem {
    pub fn builder() -> MaintenanceListProductionRequestSortItemBuilder {
        <MaintenanceListProductionRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MaintenanceListProductionRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<MaintenanceListProductionRequestSortItemDir>,
}

impl MaintenanceListProductionRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: MaintenanceListProductionRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MaintenanceListProductionRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](MaintenanceListProductionRequestSortItemBuilder::field)
    pub fn build(self) -> Result<MaintenanceListProductionRequestSortItem, BuildError> {
        Ok(MaintenanceListProductionRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
