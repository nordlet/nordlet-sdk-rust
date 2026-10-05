pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtAnnualReturnGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl MtAnnualReturnGenerateDeclarationsRequest {
    pub fn builder() -> MtAnnualReturnGenerateDeclarationsRequestBuilder {
        <MtAnnualReturnGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtAnnualReturnGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl MtAnnualReturnGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MtAnnualReturnGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](MtAnnualReturnGenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<MtAnnualReturnGenerateDeclarationsRequest, BuildError> {
        Ok(MtAnnualReturnGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
