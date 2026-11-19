pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatLiabilitiesListDeclarationsResponse {
    #[serde(default)]
    pub liabilities: Vec<GbVatLiabilitiesListDeclarationsResponseLiabilitiesItem>,
}

impl GbVatLiabilitiesListDeclarationsResponse {
    pub fn builder() -> GbVatLiabilitiesListDeclarationsResponseBuilder {
        <GbVatLiabilitiesListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatLiabilitiesListDeclarationsResponseBuilder {
    liabilities: Option<Vec<GbVatLiabilitiesListDeclarationsResponseLiabilitiesItem>>,
}

impl GbVatLiabilitiesListDeclarationsResponseBuilder {
    pub fn liabilities(
        mut self,
        value: Vec<GbVatLiabilitiesListDeclarationsResponseLiabilitiesItem>,
    ) -> Self {
        self.liabilities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatLiabilitiesListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`liabilities`](GbVatLiabilitiesListDeclarationsResponseBuilder::liabilities)
    pub fn build(self) -> Result<GbVatLiabilitiesListDeclarationsResponse, BuildError> {
        Ok(GbVatLiabilitiesListDeclarationsResponse {
            liabilities: self
                .liabilities
                .ok_or_else(|| BuildError::missing_field("liabilities"))?,
        })
    }
}
