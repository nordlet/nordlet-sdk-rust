pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtCompanyTaxGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
}

impl MtCompanyTaxGenerateDeclarationsRequest {
    pub fn builder() -> MtCompanyTaxGenerateDeclarationsRequestBuilder {
        <MtCompanyTaxGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtCompanyTaxGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
}

impl MtCompanyTaxGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MtCompanyTaxGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](MtCompanyTaxGenerateDeclarationsRequestBuilder::year)
    pub fn build(self) -> Result<MtCompanyTaxGenerateDeclarationsRequest, BuildError> {
        Ok(MtCompanyTaxGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
        })
    }
}
