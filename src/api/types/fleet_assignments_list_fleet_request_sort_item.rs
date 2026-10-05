pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignmentsListFleetRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<AssignmentsListFleetRequestSortItemDir>,
}

impl AssignmentsListFleetRequestSortItem {
    pub fn builder() -> AssignmentsListFleetRequestSortItemBuilder {
        <AssignmentsListFleetRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignmentsListFleetRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<AssignmentsListFleetRequestSortItemDir>,
}

impl AssignmentsListFleetRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: AssignmentsListFleetRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssignmentsListFleetRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AssignmentsListFleetRequestSortItemBuilder::field)
    pub fn build(self) -> Result<AssignmentsListFleetRequestSortItem, BuildError> {
        Ok(AssignmentsListFleetRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
