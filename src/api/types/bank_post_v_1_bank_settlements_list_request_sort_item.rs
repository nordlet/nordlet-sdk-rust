pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1BankSettlementsListRequestSortItemDir>,
}

impl PostV1BankSettlementsListRequestSortItem {
    pub fn builder() -> PostV1BankSettlementsListRequestSortItemBuilder {
        <PostV1BankSettlementsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1BankSettlementsListRequestSortItemDir>,
}

impl PostV1BankSettlementsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1BankSettlementsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankSettlementsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1BankSettlementsListRequestSortItem, BuildError> {
        Ok(PostV1BankSettlementsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
