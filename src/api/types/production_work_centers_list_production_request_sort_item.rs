pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkCentersListProductionRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<WorkCentersListProductionRequestSortItemDir>,
}

impl WorkCentersListProductionRequestSortItem {
    pub fn builder() -> WorkCentersListProductionRequestSortItemBuilder {
        <WorkCentersListProductionRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkCentersListProductionRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<WorkCentersListProductionRequestSortItemDir>,
}

impl WorkCentersListProductionRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: WorkCentersListProductionRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkCentersListProductionRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](WorkCentersListProductionRequestSortItemBuilder::field)
    pub fn build(self) -> Result<WorkCentersListProductionRequestSortItem, BuildError> {
        Ok(WorkCentersListProductionRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
