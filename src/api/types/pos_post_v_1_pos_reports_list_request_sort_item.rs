pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosReportsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PosReportsListRequestSortItemDir>,
}

impl PostV1PosReportsListRequestSortItem {
    pub fn builder() -> PostV1PosReportsListRequestSortItemBuilder {
        <PostV1PosReportsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PosReportsListRequestSortItemDir>,
}

impl PostV1PosReportsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PosReportsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosReportsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PosReportsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PosReportsListRequestSortItem, BuildError> {
        Ok(PostV1PosReportsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
