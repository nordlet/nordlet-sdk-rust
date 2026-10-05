pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItem {
    #[serde(default)]
    pub id: String,
    pub kind: AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemKind,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    pub size_bytes: i64,
    #[serde(rename = "storageKey")]
    #[serde(default)]
    pub storage_key: String,
}

impl AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItem {
    pub fn builder() -> AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder {
        <AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder {
    id: Option<String>,
    kind: Option<AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemKind>,
    name: Option<String>,
    file_id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    storage_key: Option<String>,
}

impl AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
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

    pub fn storage_key(mut self, value: impl Into<String>) -> Self {
        self.storage_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder::id)
    /// - [`kind`](AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder::kind)
    /// - [`name`](AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder::name)
    /// - [`file_id`](AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder::file_id)
    /// - [`file_name`](AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder::file_name)
    /// - [`mime_type`](AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder::mime_type)
    /// - [`size_bytes`](AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder::size_bytes)
    /// - [`storage_key`](AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItemBuilder::storage_key)
    pub fn build(
        self,
    ) -> Result<AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItem, BuildError> {
        Ok(
            AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItem {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                file_id: self
                    .file_id
                    .ok_or_else(|| BuildError::missing_field("file_id"))?,
                file_name: self
                    .file_name
                    .ok_or_else(|| BuildError::missing_field("file_name"))?,
                mime_type: self
                    .mime_type
                    .ok_or_else(|| BuildError::missing_field("mime_type"))?,
                size_bytes: self
                    .size_bytes
                    .ok_or_else(|| BuildError::missing_field("size_bytes"))?,
                storage_key: self
                    .storage_key
                    .ok_or_else(|| BuildError::missing_field("storage_key"))?,
            },
        )
    }
}
