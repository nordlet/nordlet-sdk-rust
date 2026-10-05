pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtCountiesListReferenceResponse {
    #[serde(default)]
    pub rows: Vec<LtCountiesListReferenceResponseRowsItem>,
}

impl LtCountiesListReferenceResponse {
    pub fn builder() -> LtCountiesListReferenceResponseBuilder {
        <LtCountiesListReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtCountiesListReferenceResponseBuilder {
    rows: Option<Vec<LtCountiesListReferenceResponseRowsItem>>,
}

impl LtCountiesListReferenceResponseBuilder {
    pub fn rows(mut self, value: Vec<LtCountiesListReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtCountiesListReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](LtCountiesListReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<LtCountiesListReferenceResponse, BuildError> {
        Ok(LtCountiesListReferenceResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
