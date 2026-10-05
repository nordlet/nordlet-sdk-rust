pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlCit8GenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl PlCit8GenerateDeclarationsRequest {
    pub fn builder() -> PlCit8GenerateDeclarationsRequestBuilder {
        <PlCit8GenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlCit8GenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl PlCit8GenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlCit8GenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlCit8GenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<PlCit8GenerateDeclarationsRequest, BuildError> {
        Ok(PlCit8GenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
