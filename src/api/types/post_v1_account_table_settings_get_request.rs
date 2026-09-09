pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountTableSettingsGetRequest {
    #[serde(rename = "tableKey")]
    #[serde(default)]
    pub table_key: String,
}

impl PostV1AccountTableSettingsGetRequest {
    pub fn builder() -> PostV1AccountTableSettingsGetRequestBuilder {
        <PostV1AccountTableSettingsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountTableSettingsGetRequestBuilder {
    table_key: Option<String>,
}

impl PostV1AccountTableSettingsGetRequestBuilder {
    pub fn table_key(mut self, value: impl Into<String>) -> Self {
        self.table_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountTableSettingsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`table_key`](PostV1AccountTableSettingsGetRequestBuilder::table_key)
    pub fn build(self) -> Result<PostV1AccountTableSettingsGetRequest, BuildError> {
        Ok(PostV1AccountTableSettingsGetRequest {
            table_key: self
                .table_key
                .ok_or_else(|| BuildError::missing_field("table_key"))?,
        })
    }
}
