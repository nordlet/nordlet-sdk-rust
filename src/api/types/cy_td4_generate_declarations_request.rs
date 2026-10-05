pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CyTd4GenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl CyTd4GenerateDeclarationsRequest {
    pub fn builder() -> CyTd4GenerateDeclarationsRequestBuilder {
        <CyTd4GenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CyTd4GenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl CyTd4GenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CyTd4GenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](CyTd4GenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<CyTd4GenerateDeclarationsRequest, BuildError> {
        Ok(CyTd4GenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
