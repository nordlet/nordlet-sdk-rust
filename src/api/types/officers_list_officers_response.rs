pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOfficersResponse {
    #[serde(default)]
    pub rows: Vec<ListOfficersResponseRowsItem>,
}

impl ListOfficersResponse {
    pub fn builder() -> ListOfficersResponseBuilder {
        <ListOfficersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOfficersResponseBuilder {
    rows: Option<Vec<ListOfficersResponseRowsItem>>,
}

impl ListOfficersResponseBuilder {
    pub fn rows(mut self, value: Vec<ListOfficersResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOfficersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ListOfficersResponseBuilder::rows)
    pub fn build(self) -> Result<ListOfficersResponse, BuildError> {
        Ok(ListOfficersResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
