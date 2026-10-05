pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuVatRatesListReferenceResponse {
    #[serde(default)]
    pub notice: String,
    #[serde(default)]
    pub rows: Vec<EuVatRatesListReferenceResponseRowsItem>,
}

impl EuVatRatesListReferenceResponse {
    pub fn builder() -> EuVatRatesListReferenceResponseBuilder {
        <EuVatRatesListReferenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatRatesListReferenceResponseBuilder {
    notice: Option<String>,
    rows: Option<Vec<EuVatRatesListReferenceResponseRowsItem>>,
}

impl EuVatRatesListReferenceResponseBuilder {
    pub fn notice(mut self, value: impl Into<String>) -> Self {
        self.notice = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<EuVatRatesListReferenceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuVatRatesListReferenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`notice`](EuVatRatesListReferenceResponseBuilder::notice)
    /// - [`rows`](EuVatRatesListReferenceResponseBuilder::rows)
    pub fn build(self) -> Result<EuVatRatesListReferenceResponse, BuildError> {
        Ok(EuVatRatesListReferenceResponse {
            notice: self
                .notice
                .ok_or_else(|| BuildError::missing_field("notice"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
