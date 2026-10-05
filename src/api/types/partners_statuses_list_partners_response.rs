pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatusesListPartnersResponse {
    #[serde(default)]
    pub rows: Vec<StatusesListPartnersResponseRowsItem>,
}

impl StatusesListPartnersResponse {
    pub fn builder() -> StatusesListPartnersResponseBuilder {
        <StatusesListPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusesListPartnersResponseBuilder {
    rows: Option<Vec<StatusesListPartnersResponseRowsItem>>,
}

impl StatusesListPartnersResponseBuilder {
    pub fn rows(mut self, value: Vec<StatusesListPartnersResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatusesListPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](StatusesListPartnersResponseBuilder::rows)
    pub fn build(self) -> Result<StatusesListPartnersResponse, BuildError> {
        Ok(StatusesListPartnersResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
