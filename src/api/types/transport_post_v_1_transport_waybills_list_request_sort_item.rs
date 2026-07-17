pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1TransportWaybillsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1TransportWaybillsListRequestSortItemDir>,
}

impl PostV1TransportWaybillsListRequestSortItem {
    pub fn builder() -> PostV1TransportWaybillsListRequestSortItemBuilder {
        <PostV1TransportWaybillsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1TransportWaybillsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1TransportWaybillsListRequestSortItemDir>,
}

impl PostV1TransportWaybillsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1TransportWaybillsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1TransportWaybillsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1TransportWaybillsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1TransportWaybillsListRequestSortItem, BuildError> {
        Ok(PostV1TransportWaybillsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
