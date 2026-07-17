pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AuditListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1AuditListRequestSortItemDir>,
}

impl PostV1AuditListRequestSortItem {
    pub fn builder() -> PostV1AuditListRequestSortItemBuilder {
        <PostV1AuditListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AuditListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1AuditListRequestSortItemDir>,
}

impl PostV1AuditListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1AuditListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AuditListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AuditListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1AuditListRequestSortItem, BuildError> {
        Ok(PostV1AuditListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
