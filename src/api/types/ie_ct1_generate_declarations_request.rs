pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeCt1GenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl IeCt1GenerateDeclarationsRequest {
    pub fn builder() -> IeCt1GenerateDeclarationsRequestBuilder {
        <IeCt1GenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeCt1GenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl IeCt1GenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IeCt1GenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](IeCt1GenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<IeCt1GenerateDeclarationsRequest, BuildError> {
        Ok(IeCt1GenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
