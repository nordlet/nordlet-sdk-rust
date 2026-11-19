pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbVatObligationsListDeclarationsResponse {
    #[serde(default)]
    pub obligations: Vec<GbVatObligationsListDeclarationsResponseObligationsItem>,
}

impl GbVatObligationsListDeclarationsResponse {
    pub fn builder() -> GbVatObligationsListDeclarationsResponseBuilder {
        <GbVatObligationsListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatObligationsListDeclarationsResponseBuilder {
    obligations: Option<Vec<GbVatObligationsListDeclarationsResponseObligationsItem>>,
}

impl GbVatObligationsListDeclarationsResponseBuilder {
    pub fn obligations(
        mut self,
        value: Vec<GbVatObligationsListDeclarationsResponseObligationsItem>,
    ) -> Self {
        self.obligations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatObligationsListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`obligations`](GbVatObligationsListDeclarationsResponseBuilder::obligations)
    pub fn build(self) -> Result<GbVatObligationsListDeclarationsResponse, BuildError> {
        Ok(GbVatObligationsListDeclarationsResponse {
            obligations: self
                .obligations
                .ok_or_else(|| BuildError::missing_field("obligations"))?,
        })
    }
}
