pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesListAccountResponse {
    #[serde(default)]
    pub rows: Vec<InvitesListAccountResponseRowsItem>,
}

impl InvitesListAccountResponse {
    pub fn builder() -> InvitesListAccountResponseBuilder {
        <InvitesListAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesListAccountResponseBuilder {
    rows: Option<Vec<InvitesListAccountResponseRowsItem>>,
}

impl InvitesListAccountResponseBuilder {
    pub fn rows(mut self, value: Vec<InvitesListAccountResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitesListAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](InvitesListAccountResponseBuilder::rows)
    pub fn build(self) -> Result<InvitesListAccountResponse, BuildError> {
        Ok(InvitesListAccountResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
