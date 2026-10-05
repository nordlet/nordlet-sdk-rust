pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuVatReturnPacksListDeclarationsResponse {
    #[serde(default)]
    pub packs: Vec<EuVatReturnPacksListDeclarationsResponsePacksItem>,
}

impl EuVatReturnPacksListDeclarationsResponse {
    pub fn builder() -> EuVatReturnPacksListDeclarationsResponseBuilder {
        <EuVatReturnPacksListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatReturnPacksListDeclarationsResponseBuilder {
    packs: Option<Vec<EuVatReturnPacksListDeclarationsResponsePacksItem>>,
}

impl EuVatReturnPacksListDeclarationsResponseBuilder {
    pub fn packs(mut self, value: Vec<EuVatReturnPacksListDeclarationsResponsePacksItem>) -> Self {
        self.packs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuVatReturnPacksListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`packs`](EuVatReturnPacksListDeclarationsResponseBuilder::packs)
    pub fn build(self) -> Result<EuVatReturnPacksListDeclarationsResponse, BuildError> {
        Ok(EuVatReturnPacksListDeclarationsResponse {
            packs: self
                .packs
                .ok_or_else(|| BuildError::missing_field("packs"))?,
        })
    }
}
