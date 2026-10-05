pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobsGetReportsResponseOutputsItem {
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

impl JobsGetReportsResponseOutputsItem {
    pub fn builder() -> JobsGetReportsResponseOutputsItemBuilder {
        <JobsGetReportsResponseOutputsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsGetReportsResponseOutputsItemBuilder {
    format: Option<String>,
    file_id: Option<String>,
    file_name: Option<String>,
    size_bytes: Option<i64>,
}

impl JobsGetReportsResponseOutputsItemBuilder {
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

    /// Consumes the builder and constructs a [`JobsGetReportsResponseOutputsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](JobsGetReportsResponseOutputsItemBuilder::format)
    /// - [`file_id`](JobsGetReportsResponseOutputsItemBuilder::file_id)
    /// - [`file_name`](JobsGetReportsResponseOutputsItemBuilder::file_name)
    /// - [`size_bytes`](JobsGetReportsResponseOutputsItemBuilder::size_bytes)
    pub fn build(self) -> Result<JobsGetReportsResponseOutputsItem, BuildError> {
        Ok(JobsGetReportsResponseOutputsItem {
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
