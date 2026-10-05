pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtRegionsListReferenceResponse {
    #[serde(default)]
    pub rows: Vec<LtRegionsListReferenceResponseRowsItem>,
}

impl LtRegionsListReferenceResponse {
    pub fn builder() -> LtRegionsListReferenceResponseBuilder {
        <LtRegionsListReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtRegionsListReferenceResponseBuilder {
    rows: Option<Vec<LtRegionsListReferenceResponseRowsItem>>,
}

impl LtRegionsListReferenceResponseBuilder {
    pub fn rows(mut self, value: Vec<LtRegionsListReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtRegionsListReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](LtRegionsListReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<LtRegionsListReferenceResponse, BuildError> {
        Ok(LtRegionsListReferenceResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
