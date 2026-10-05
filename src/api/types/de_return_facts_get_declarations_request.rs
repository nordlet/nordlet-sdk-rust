pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsGetDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl DeReturnFactsGetDeclarationsRequest {
    pub fn builder() -> DeReturnFactsGetDeclarationsRequestBuilder {
        <DeReturnFactsGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsGetDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl DeReturnFactsGetDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeReturnFactsGetDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<DeReturnFactsGetDeclarationsRequest, BuildError> {
        Ok(DeReturnFactsGetDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
