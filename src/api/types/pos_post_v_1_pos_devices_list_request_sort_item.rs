pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosDevicesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PosDevicesListRequestSortItemDir>,
}

impl PostV1PosDevicesListRequestSortItem {
    pub fn builder() -> PostV1PosDevicesListRequestSortItemBuilder {
        <PostV1PosDevicesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosDevicesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PosDevicesListRequestSortItemDir>,
}

impl PostV1PosDevicesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PosDevicesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosDevicesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PosDevicesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PosDevicesListRequestSortItem, BuildError> {
        Ok(PostV1PosDevicesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
