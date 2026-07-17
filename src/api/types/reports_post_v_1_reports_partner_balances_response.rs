pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsPartnerBalancesResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReportsPartnerBalancesResponseRowsItem>,
}

impl PostV1ReportsPartnerBalancesResponse {
    pub fn builder() -> PostV1ReportsPartnerBalancesResponseBuilder {
        <PostV1ReportsPartnerBalancesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsPartnerBalancesResponseBuilder {
    rows: Option<Vec<PostV1ReportsPartnerBalancesResponseRowsItem>>,
}

impl PostV1ReportsPartnerBalancesResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReportsPartnerBalancesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsPartnerBalancesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReportsPartnerBalancesResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReportsPartnerBalancesResponse, BuildError> {
        Ok(PostV1ReportsPartnerBalancesResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
