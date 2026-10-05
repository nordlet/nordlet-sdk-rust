pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeB1GenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl IeB1GenerateDeclarationsRequest {
    pub fn builder() -> IeB1GenerateDeclarationsRequestBuilder {
        <IeB1GenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeB1GenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl IeB1GenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IeB1GenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](IeB1GenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<IeB1GenerateDeclarationsRequest, BuildError> {
        Ok(IeB1GenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
