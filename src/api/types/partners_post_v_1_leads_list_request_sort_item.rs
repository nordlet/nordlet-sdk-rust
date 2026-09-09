pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1LeadsListRequestSortItemDir>,
}

impl PostV1LeadsListRequestSortItem {
    pub fn builder() -> PostV1LeadsListRequestSortItemBuilder {
        <PostV1LeadsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1LeadsListRequestSortItemDir>,
}

impl PostV1LeadsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1LeadsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LeadsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1LeadsListRequestSortItem, BuildError> {
        Ok(PostV1LeadsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
