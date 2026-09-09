pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountTableSettingsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1AccountTableSettingsListResponseRowsItem>,
}

impl PostV1AccountTableSettingsListResponse {
    pub fn builder() -> PostV1AccountTableSettingsListResponseBuilder {
        <PostV1AccountTableSettingsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountTableSettingsListResponseBuilder {
    rows: Option<Vec<PostV1AccountTableSettingsListResponseRowsItem>>,
}

impl PostV1AccountTableSettingsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1AccountTableSettingsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountTableSettingsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1AccountTableSettingsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1AccountTableSettingsListResponse, BuildError> {
        Ok(PostV1AccountTableSettingsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
