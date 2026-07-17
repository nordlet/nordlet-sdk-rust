pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1HrEmployeesListRequestSortItemDir>,
}

impl PostV1HrEmployeesListRequestSortItem {
    pub fn builder() -> PostV1HrEmployeesListRequestSortItemBuilder {
        <PostV1HrEmployeesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1HrEmployeesListRequestSortItemDir>,
}

impl PostV1HrEmployeesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1HrEmployeesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrEmployeesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1HrEmployeesListRequestSortItem, BuildError> {
        Ok(PostV1HrEmployeesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
