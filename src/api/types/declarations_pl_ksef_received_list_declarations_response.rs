pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlKsefReceivedListDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<PlKsefReceivedListDeclarationsResponseRowsItem>,
}

impl PlKsefReceivedListDeclarationsResponse {
    pub fn builder() -> PlKsefReceivedListDeclarationsResponseBuilder {
        <PlKsefReceivedListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlKsefReceivedListDeclarationsResponseBuilder {
    rows: Option<Vec<PlKsefReceivedListDeclarationsResponseRowsItem>>,
}

impl PlKsefReceivedListDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<PlKsefReceivedListDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlKsefReceivedListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PlKsefReceivedListDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<PlKsefReceivedListDeclarationsResponse, BuildError> {
        Ok(PlKsefReceivedListDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
