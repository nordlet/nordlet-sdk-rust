pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Us940GetDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl Us940GetDeclarationsRequest {
    pub fn builder() -> Us940GetDeclarationsRequestBuilder {
        <Us940GetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us940GetDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl Us940GetDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Us940GetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](Us940GetDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<Us940GetDeclarationsRequest, BuildError> {
        Ok(Us940GetDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
