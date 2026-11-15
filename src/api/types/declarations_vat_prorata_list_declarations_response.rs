pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatProrataListDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<VatProrataListDeclarationsResponseRowsItem>,
}

impl VatProrataListDeclarationsResponse {
    pub fn builder() -> VatProrataListDeclarationsResponseBuilder {
        <VatProrataListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatProrataListDeclarationsResponseBuilder {
    rows: Option<Vec<VatProrataListDeclarationsResponseRowsItem>>,
}

impl VatProrataListDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<VatProrataListDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatProrataListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](VatProrataListDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<VatProrataListDeclarationsResponse, BuildError> {
        Ok(VatProrataListDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
