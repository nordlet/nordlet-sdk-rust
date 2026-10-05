pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlVatUeGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PlVatUeGenerateDeclarationsRequest {
    pub fn builder() -> PlVatUeGenerateDeclarationsRequestBuilder {
        <PlVatUeGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlVatUeGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PlVatUeGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlVatUeGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlVatUeGenerateDeclarationsRequestBuilder::year)
    /// - [`month`](PlVatUeGenerateDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<PlVatUeGenerateDeclarationsRequest, BuildError> {
        Ok(PlVatUeGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
