pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDac7PreviewDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl EuDac7PreviewDeclarationsRequest {
    pub fn builder() -> EuDac7PreviewDeclarationsRequestBuilder {
        <EuDac7PreviewDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDac7PreviewDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl EuDac7PreviewDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuDac7PreviewDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuDac7PreviewDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<EuDac7PreviewDeclarationsRequest, BuildError> {
        Ok(EuDac7PreviewDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
