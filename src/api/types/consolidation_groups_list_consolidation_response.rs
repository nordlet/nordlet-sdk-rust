pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsListConsolidationResponse {
    #[serde(default)]
    pub rows: Vec<GroupsListConsolidationResponseRowsItem>,
}

impl GroupsListConsolidationResponse {
    pub fn builder() -> GroupsListConsolidationResponseBuilder {
        <GroupsListConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsListConsolidationResponseBuilder {
    rows: Option<Vec<GroupsListConsolidationResponseRowsItem>>,
}

impl GroupsListConsolidationResponseBuilder {
    pub fn rows(mut self, value: Vec<GroupsListConsolidationResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupsListConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](GroupsListConsolidationResponseBuilder::rows)
    pub fn build(self) -> Result<GroupsListConsolidationResponse, BuildError> {
        Ok(GroupsListConsolidationResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
