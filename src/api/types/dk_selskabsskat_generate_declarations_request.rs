pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DkSelskabsskatGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl DkSelskabsskatGenerateDeclarationsRequest {
    pub fn builder() -> DkSelskabsskatGenerateDeclarationsRequestBuilder {
        <DkSelskabsskatGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DkSelskabsskatGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl DkSelskabsskatGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DkSelskabsskatGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DkSelskabsskatGenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<DkSelskabsskatGenerateDeclarationsRequest, BuildError> {
        Ok(DkSelskabsskatGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
