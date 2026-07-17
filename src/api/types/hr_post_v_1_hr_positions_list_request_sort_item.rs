pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrPositionsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1HrPositionsListRequestSortItemDir>,
}

impl PostV1HrPositionsListRequestSortItem {
    pub fn builder() -> PostV1HrPositionsListRequestSortItemBuilder {
        <PostV1HrPositionsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1HrPositionsListRequestSortItemDir>,
}

impl PostV1HrPositionsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1HrPositionsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrPositionsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1HrPositionsListRequestSortItem, BuildError> {
        Ok(PostV1HrPositionsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
