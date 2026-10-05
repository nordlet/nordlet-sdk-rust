pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesAttachmentsListHrResponseRowsItem {
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl EmployeesAttachmentsListHrResponseRowsItem {
    pub fn builder() -> EmployeesAttachmentsListHrResponseRowsItemBuilder {
        <EmployeesAttachmentsListHrResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesAttachmentsListHrResponseRowsItemBuilder {
    id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl EmployeesAttachmentsListHrResponseRowsItemBuilder {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesAttachmentsListHrResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesAttachmentsListHrResponseRowsItemBuilder::id)
    /// - [`file_name`](EmployeesAttachmentsListHrResponseRowsItemBuilder::file_name)
    /// - [`mime_type`](EmployeesAttachmentsListHrResponseRowsItemBuilder::mime_type)
    /// - [`size_bytes`](EmployeesAttachmentsListHrResponseRowsItemBuilder::size_bytes)
    /// - [`created_at`](EmployeesAttachmentsListHrResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<EmployeesAttachmentsListHrResponseRowsItem, BuildError> {
        Ok(EmployeesAttachmentsListHrResponseRowsItem {
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
