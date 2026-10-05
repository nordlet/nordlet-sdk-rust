pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CyHe32GenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl CyHe32GenerateDeclarationsRequest {
    pub fn builder() -> CyHe32GenerateDeclarationsRequestBuilder {
        <CyHe32GenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CyHe32GenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl CyHe32GenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CyHe32GenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](CyHe32GenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<CyHe32GenerateDeclarationsRequest, BuildError> {
        Ok(CyHe32GenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
