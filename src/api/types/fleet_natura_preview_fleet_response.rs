pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NaturaPreviewFleetResponse {
    #[serde(default)]
    pub rows: Vec<NaturaPreviewFleetResponseRowsItem>,
    #[serde(default)]
    pub total: String,
}

impl NaturaPreviewFleetResponse {
    pub fn builder() -> NaturaPreviewFleetResponseBuilder {
        <NaturaPreviewFleetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NaturaPreviewFleetResponseBuilder {
    rows: Option<Vec<NaturaPreviewFleetResponseRowsItem>>,
    total: Option<String>,
}

impl NaturaPreviewFleetResponseBuilder {
    pub fn rows(mut self, value: Vec<NaturaPreviewFleetResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NaturaPreviewFleetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](NaturaPreviewFleetResponseBuilder::rows)
    /// - [`total`](NaturaPreviewFleetResponseBuilder::total)
    pub fn build(self) -> Result<NaturaPreviewFleetResponse, BuildError> {
        Ok(NaturaPreviewFleetResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
