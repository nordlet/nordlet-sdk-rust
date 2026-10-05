pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsListAccountResponse {
    #[serde(default)]
    pub rows: Vec<SessionsListAccountResponseRowsItem>,
}

impl SessionsListAccountResponse {
    pub fn builder() -> SessionsListAccountResponseBuilder {
        <SessionsListAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsListAccountResponseBuilder {
    rows: Option<Vec<SessionsListAccountResponseRowsItem>>,
}

impl SessionsListAccountResponseBuilder {
    pub fn rows(mut self, value: Vec<SessionsListAccountResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionsListAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](SessionsListAccountResponseBuilder::rows)
    pub fn build(self) -> Result<SessionsListAccountResponse, BuildError> {
        Ok(SessionsListAccountResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
