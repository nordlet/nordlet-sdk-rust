pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsListPartnersResponse {
    #[serde(default)]
    pub rows: Vec<GroupsListPartnersResponseRowsItem>,
}

impl GroupsListPartnersResponse {
    pub fn builder() -> GroupsListPartnersResponseBuilder {
        <GroupsListPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsListPartnersResponseBuilder {
    rows: Option<Vec<GroupsListPartnersResponseRowsItem>>,
}

impl GroupsListPartnersResponseBuilder {
    pub fn rows(mut self, value: Vec<GroupsListPartnersResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupsListPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](GroupsListPartnersResponseBuilder::rows)
    pub fn build(self) -> Result<GroupsListPartnersResponse, BuildError> {
        Ok(GroupsListPartnersResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
