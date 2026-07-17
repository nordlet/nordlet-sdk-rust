pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesAttachmentsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    pub size_bytes: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1HrEmployeesAttachmentsListResponseRowsItem {
    pub fn builder() -> PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder {
        <PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder {
    id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    created_at: Option<String>,
}

impl PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesAttachmentsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder::id)
    /// - [`file_name`](PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder::file_name)
    /// - [`mime_type`](PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder::mime_type)
    /// - [`size_bytes`](PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder::size_bytes)
    /// - [`created_at`](PostV1HrEmployeesAttachmentsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1HrEmployeesAttachmentsListResponseRowsItem, BuildError> {
        Ok(PostV1HrEmployeesAttachmentsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
            size_bytes: self
                .size_bytes
                .ok_or_else(|| BuildError::missing_field("size_bytes"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
