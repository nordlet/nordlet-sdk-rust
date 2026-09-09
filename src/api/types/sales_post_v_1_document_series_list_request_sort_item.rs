pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DocumentSeriesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1DocumentSeriesListRequestSortItemDir>,
}

impl PostV1DocumentSeriesListRequestSortItem {
    pub fn builder() -> PostV1DocumentSeriesListRequestSortItemBuilder {
        <PostV1DocumentSeriesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DocumentSeriesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1DocumentSeriesListRequestSortItemDir>,
}

impl PostV1DocumentSeriesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1DocumentSeriesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DocumentSeriesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DocumentSeriesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1DocumentSeriesListRequestSortItem, BuildError> {
        Ok(PostV1DocumentSeriesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
