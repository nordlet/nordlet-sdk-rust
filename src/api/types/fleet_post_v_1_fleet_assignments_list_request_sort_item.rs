pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetAssignmentsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1FleetAssignmentsListRequestSortItemDir>,
}

impl PostV1FleetAssignmentsListRequestSortItem {
    pub fn builder() -> PostV1FleetAssignmentsListRequestSortItemBuilder {
        <PostV1FleetAssignmentsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetAssignmentsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1FleetAssignmentsListRequestSortItemDir>,
}

impl PostV1FleetAssignmentsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1FleetAssignmentsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetAssignmentsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1FleetAssignmentsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1FleetAssignmentsListRequestSortItem, BuildError> {
        Ok(PostV1FleetAssignmentsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
