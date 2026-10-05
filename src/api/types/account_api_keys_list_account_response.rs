pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysListAccountResponse {
    #[serde(default)]
    pub rows: Vec<ApiKeysListAccountResponseRowsItem>,
}

impl ApiKeysListAccountResponse {
    pub fn builder() -> ApiKeysListAccountResponseBuilder {
        <ApiKeysListAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysListAccountResponseBuilder {
    rows: Option<Vec<ApiKeysListAccountResponseRowsItem>>,
}

impl ApiKeysListAccountResponseBuilder {
    pub fn rows(mut self, value: Vec<ApiKeysListAccountResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysListAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ApiKeysListAccountResponseBuilder::rows)
    pub fn build(self) -> Result<ApiKeysListAccountResponse, BuildError> {
        Ok(ApiKeysListAccountResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
