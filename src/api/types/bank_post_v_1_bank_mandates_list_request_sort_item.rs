pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMandatesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1BankMandatesListRequestSortItemDir>,
}

impl PostV1BankMandatesListRequestSortItem {
    pub fn builder() -> PostV1BankMandatesListRequestSortItemBuilder {
        <PostV1BankMandatesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMandatesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1BankMandatesListRequestSortItemDir>,
}

impl PostV1BankMandatesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1BankMandatesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankMandatesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankMandatesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1BankMandatesListRequestSortItem, BuildError> {
        Ok(PostV1BankMandatesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
