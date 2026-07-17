pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCnCodesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ReferenceCnCodesListRequestSortItemDir>,
}

impl PostV1ReferenceCnCodesListRequestSortItem {
    pub fn builder() -> PostV1ReferenceCnCodesListRequestSortItemBuilder {
        <PostV1ReferenceCnCodesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCnCodesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ReferenceCnCodesListRequestSortItemDir>,
}

impl PostV1ReferenceCnCodesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ReferenceCnCodesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCnCodesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceCnCodesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ReferenceCnCodesListRequestSortItem, BuildError> {
        Ok(PostV1ReferenceCnCodesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
