pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeEuerGetDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl DeEuerGetDeclarationsRequest {
    pub fn builder() -> DeEuerGetDeclarationsRequestBuilder {
        <DeEuerGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeEuerGetDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl DeEuerGetDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeEuerGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeEuerGetDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<DeEuerGetDeclarationsRequest, BuildError> {
        Ok(DeEuerGetDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
