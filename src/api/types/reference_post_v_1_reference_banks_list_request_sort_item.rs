pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceBanksListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ReferenceBanksListRequestSortItemDir>,
}

impl PostV1ReferenceBanksListRequestSortItem {
    pub fn builder() -> PostV1ReferenceBanksListRequestSortItemBuilder {
        <PostV1ReferenceBanksListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceBanksListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ReferenceBanksListRequestSortItemDir>,
}

impl PostV1ReferenceBanksListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ReferenceBanksListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceBanksListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceBanksListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ReferenceBanksListRequestSortItem, BuildError> {
        Ok(PostV1ReferenceBanksListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
