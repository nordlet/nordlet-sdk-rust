pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntrastatThresholdsListReferenceResponse {
    #[serde(default)]
    pub rows: Vec<IntrastatThresholdsListReferenceResponseRowsItem>,
}

impl IntrastatThresholdsListReferenceResponse {
    pub fn builder() -> IntrastatThresholdsListReferenceResponseBuilder {
        <IntrastatThresholdsListReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntrastatThresholdsListReferenceResponseBuilder {
    rows: Option<Vec<IntrastatThresholdsListReferenceResponseRowsItem>>,
}

impl IntrastatThresholdsListReferenceResponseBuilder {
    pub fn rows(mut self, value: Vec<IntrastatThresholdsListReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntrastatThresholdsListReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](IntrastatThresholdsListReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<IntrastatThresholdsListReferenceResponse, BuildError> {
        Ok(IntrastatThresholdsListReferenceResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
