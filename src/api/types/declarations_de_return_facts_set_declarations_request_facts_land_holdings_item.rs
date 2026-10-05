pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItem {
    #[serde(rename = "fileNumber")]
    #[serde(default)]
    pub file_number: String,
    #[serde(rename = "assessedValue")]
    #[serde(default)]
    pub assessed_value: String,
    pub category: DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemCategory,
}

impl DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemBuilder {
        <DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemBuilder {
    file_number: Option<String>,
    assessed_value: Option<String>,
    category: Option<DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemCategory>,
}

impl DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemBuilder {
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
        value: DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_number`](DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemBuilder::file_number)
    /// - [`assessed_value`](DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemBuilder::assessed_value)
    /// - [`category`](DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItemBuilder::category)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItem, BuildError> {
        Ok(DeReturnFactsSetDeclarationsRequestFactsLandHoldingsItem {
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
