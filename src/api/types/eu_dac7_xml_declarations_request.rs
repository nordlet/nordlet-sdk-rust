pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDac7XmlDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl EuDac7XmlDeclarationsRequest {
    pub fn builder() -> EuDac7XmlDeclarationsRequestBuilder {
        <EuDac7XmlDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDac7XmlDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl EuDac7XmlDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuDac7XmlDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuDac7XmlDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<EuDac7XmlDeclarationsRequest, BuildError> {
        Ok(EuDac7XmlDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
