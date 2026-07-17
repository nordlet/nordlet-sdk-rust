pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesRecordsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1HrEmployeesRecordsListRequestSortItemDir>,
}

impl PostV1HrEmployeesRecordsListRequestSortItem {
    pub fn builder() -> PostV1HrEmployeesRecordsListRequestSortItemBuilder {
        <PostV1HrEmployeesRecordsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesRecordsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1HrEmployeesRecordsListRequestSortItemDir>,
}

impl PostV1HrEmployeesRecordsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1HrEmployeesRecordsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesRecordsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrEmployeesRecordsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1HrEmployeesRecordsListRequestSortItem, BuildError> {
        Ok(PostV1HrEmployeesRecordsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
