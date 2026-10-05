pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdsListDeclarationsResponseThresholdsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "nationalThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_threshold: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectors: Option<Vec<EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItem>>,
    #[serde(rename = "intraEuAcquisitionsTrigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_eu_acquisitions_trigger:
        Option<EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTrigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    pub source: String,
}

impl EuSmeThresholdsListDeclarationsResponseThresholdsItem {
    pub fn builder() -> EuSmeThresholdsListDeclarationsResponseThresholdsItemBuilder {
        <EuSmeThresholdsListDeclarationsResponseThresholdsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdsListDeclarationsResponseThresholdsItemBuilder {
    country_code: Option<String>,
    currency: Option<String>,
    national_threshold: Option<String>,
    sectors: Option<Vec<EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItem>>,
    intra_eu_acquisitions_trigger:
        Option<EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTrigger>,
    note: Option<String>,
    source: Option<String>,
}

impl EuSmeThresholdsListDeclarationsResponseThresholdsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn national_threshold(mut self, value: impl Into<String>) -> Self {
        self.national_threshold = Some(value.into());
        self
    }

    pub fn sectors(
        mut self,
        value: Vec<EuSmeThresholdsListDeclarationsResponseThresholdsItemSectorsItem>,
    ) -> Self {
        self.sectors = Some(value);
        self
    }

    pub fn intra_eu_acquisitions_trigger(
        mut self,
        value: EuSmeThresholdsListDeclarationsResponseThresholdsItemIntraEuAcquisitionsTrigger,
    ) -> Self {
        self.intra_eu_acquisitions_trigger = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuSmeThresholdsListDeclarationsResponseThresholdsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuSmeThresholdsListDeclarationsResponseThresholdsItemBuilder::country_code)
    /// - [`currency`](EuSmeThresholdsListDeclarationsResponseThresholdsItemBuilder::currency)
    /// - [`source`](EuSmeThresholdsListDeclarationsResponseThresholdsItemBuilder::source)
    pub fn build(
        self,
    ) -> Result<EuSmeThresholdsListDeclarationsResponseThresholdsItem, BuildError> {
        Ok(EuSmeThresholdsListDeclarationsResponseThresholdsItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            national_threshold: self.national_threshold,
            sectors: self.sectors,
            intra_eu_acquisitions_trigger: self.intra_eu_acquisitions_trigger,
            note: self.note,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
