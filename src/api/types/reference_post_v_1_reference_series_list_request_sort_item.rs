pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceSeriesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ReferenceSeriesListRequestSortItemDir>,
}

impl PostV1ReferenceSeriesListRequestSortItem {
    pub fn builder() -> PostV1ReferenceSeriesListRequestSortItemBuilder {
        <PostV1ReferenceSeriesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceSeriesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ReferenceSeriesListRequestSortItemDir>,
}

impl PostV1ReferenceSeriesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ReferenceSeriesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceSeriesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceSeriesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ReferenceSeriesListRequestSortItem, BuildError> {
        Ok(PostV1ReferenceSeriesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
