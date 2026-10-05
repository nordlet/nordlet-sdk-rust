pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlPit11GenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl PlPit11GenerateDeclarationsRequest {
    pub fn builder() -> PlPit11GenerateDeclarationsRequestBuilder {
        <PlPit11GenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlPit11GenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl PlPit11GenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlPit11GenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlPit11GenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<PlPit11GenerateDeclarationsRequest, BuildError> {
        Ok(PlPit11GenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
