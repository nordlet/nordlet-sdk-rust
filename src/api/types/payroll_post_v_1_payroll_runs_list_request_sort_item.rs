pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PayrollRunsListRequestSortItemDir>,
}

impl PostV1PayrollRunsListRequestSortItem {
    pub fn builder() -> PostV1PayrollRunsListRequestSortItemBuilder {
        <PostV1PayrollRunsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PayrollRunsListRequestSortItemDir>,
}

impl PostV1PayrollRunsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PayrollRunsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PayrollRunsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PayrollRunsListRequestSortItem, BuildError> {
        Ok(PostV1PayrollRunsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
