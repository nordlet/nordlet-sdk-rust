pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeDeuevGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl DeDeuevGenerateDeclarationsRequest {
    pub fn builder() -> DeDeuevGenerateDeclarationsRequestBuilder {
        <DeDeuevGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeDeuevGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl DeDeuevGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeDeuevGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DeDeuevGenerateDeclarationsRequestBuilder::year)
    /// - [`month`](DeDeuevGenerateDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<DeDeuevGenerateDeclarationsRequest, BuildError> {
        Ok(DeDeuevGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
