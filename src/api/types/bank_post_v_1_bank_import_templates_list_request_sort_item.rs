pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1BankImportTemplatesListRequestSortItemDir>,
}

impl PostV1BankImportTemplatesListRequestSortItem {
    pub fn builder() -> PostV1BankImportTemplatesListRequestSortItemBuilder {
        <PostV1BankImportTemplatesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1BankImportTemplatesListRequestSortItemDir>,
}

impl PostV1BankImportTemplatesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1BankImportTemplatesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankImportTemplatesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1BankImportTemplatesListRequestSortItem, BuildError> {
        Ok(PostV1BankImportTemplatesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
