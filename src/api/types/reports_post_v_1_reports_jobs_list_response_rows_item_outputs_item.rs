pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsJobsListResponseRowsItemOutputsItem {
    #[serde(default)]
    pub format: String,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    pub size_bytes: i64,
}

impl PostV1ReportsJobsListResponseRowsItemOutputsItem {
    pub fn builder() -> PostV1ReportsJobsListResponseRowsItemOutputsItemBuilder {
        <PostV1ReportsJobsListResponseRowsItemOutputsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsJobsListResponseRowsItemOutputsItemBuilder {
    format: Option<String>,
    file_id: Option<String>,
    file_name: Option<String>,
    size_bytes: Option<i64>,
}

impl PostV1ReportsJobsListResponseRowsItemOutputsItemBuilder {
    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
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

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsJobsListResponseRowsItemOutputsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](PostV1ReportsJobsListResponseRowsItemOutputsItemBuilder::format)
    /// - [`file_id`](PostV1ReportsJobsListResponseRowsItemOutputsItemBuilder::file_id)
    /// - [`file_name`](PostV1ReportsJobsListResponseRowsItemOutputsItemBuilder::file_name)
    /// - [`size_bytes`](PostV1ReportsJobsListResponseRowsItemOutputsItemBuilder::size_bytes)
    pub fn build(self) -> Result<PostV1ReportsJobsListResponseRowsItemOutputsItem, BuildError> {
        Ok(PostV1ReportsJobsListResponseRowsItemOutputsItem {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            size_bytes: self
                .size_bytes
                .ok_or_else(|| BuildError::missing_field("size_bytes"))?,
        })
    }
}
