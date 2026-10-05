pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersListAccountResponse {
    #[serde(default)]
    pub rows: Vec<MembersListAccountResponseRowsItem>,
}

impl MembersListAccountResponse {
    pub fn builder() -> MembersListAccountResponseBuilder {
        <MembersListAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersListAccountResponseBuilder {
    rows: Option<Vec<MembersListAccountResponseRowsItem>>,
}

impl MembersListAccountResponseBuilder {
    pub fn rows(mut self, value: Vec<MembersListAccountResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembersListAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](MembersListAccountResponseBuilder::rows)
    pub fn build(self) -> Result<MembersListAccountResponse, BuildError> {
        Ok(MembersListAccountResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
