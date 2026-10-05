pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItem {
    #[serde(rename = "fileNumber")]
    #[serde(default)]
    pub file_number: String,
    #[serde(rename = "assessedValue")]
    #[serde(default)]
    pub assessed_value: String,
    pub category: DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemCategory,
}

impl DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItem {
    pub fn builder() -> DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemBuilder {
        <DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemBuilder {
    file_number: Option<String>,
    assessed_value: Option<String>,
    category: Option<DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemCategory>,
}

impl DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemBuilder {
    pub fn file_number(mut self, value: impl Into<String>) -> Self {
        self.file_number = Some(value.into());
        self
    }

    pub fn assessed_value(mut self, value: impl Into<String>) -> Self {
        self.assessed_value = Some(value.into());
        self
    }

    pub fn category(
        mut self,
        value: DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_number`](DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemBuilder::file_number)
    /// - [`assessed_value`](DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemBuilder::assessed_value)
    /// - [`category`](DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItemBuilder::category)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItem, BuildError> {
        Ok(DeReturnFactsGetDeclarationsResponseFactsLandHoldingsItem {
            file_number: self
                .file_number
                .ok_or_else(|| BuildError::missing_field("file_number"))?,
            assessed_value: self
                .assessed_value
                .ok_or_else(|| BuildError::missing_field("assessed_value"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
        })
    }
}
