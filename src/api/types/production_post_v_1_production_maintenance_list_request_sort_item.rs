pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionMaintenanceListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ProductionMaintenanceListRequestSortItemDir>,
}

impl PostV1ProductionMaintenanceListRequestSortItem {
    pub fn builder() -> PostV1ProductionMaintenanceListRequestSortItemBuilder {
        <PostV1ProductionMaintenanceListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionMaintenanceListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ProductionMaintenanceListRequestSortItemDir>,
}

impl PostV1ProductionMaintenanceListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ProductionMaintenanceListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionMaintenanceListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionMaintenanceListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ProductionMaintenanceListRequestSortItem, BuildError> {
        Ok(PostV1ProductionMaintenanceListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
