pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobsListReportsResponseRowsItemOutputsItem {
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

impl JobsListReportsResponseRowsItemOutputsItem {
    pub fn builder() -> JobsListReportsResponseRowsItemOutputsItemBuilder {
        <JobsListReportsResponseRowsItemOutputsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsListReportsResponseRowsItemOutputsItemBuilder {
    format: Option<String>,
    file_id: Option<String>,
    file_name: Option<String>,
    size_bytes: Option<i64>,
}

impl JobsListReportsResponseRowsItemOutputsItemBuilder {
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

    /// Consumes the builder and constructs a [`JobsListReportsResponseRowsItemOutputsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](JobsListReportsResponseRowsItemOutputsItemBuilder::format)
    /// - [`file_id`](JobsListReportsResponseRowsItemOutputsItemBuilder::file_id)
    /// - [`file_name`](JobsListReportsResponseRowsItemOutputsItemBuilder::file_name)
    /// - [`size_bytes`](JobsListReportsResponseRowsItemOutputsItemBuilder::size_bytes)
    pub fn build(self) -> Result<JobsListReportsResponseRowsItemOutputsItem, BuildError> {
        Ok(JobsListReportsResponseRowsItemOutputsItem {
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
