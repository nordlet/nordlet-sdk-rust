pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdGetDeclarationsResponseThreshold {
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "nationalThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_threshold: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectors: Option<Vec<EuSmeThresholdGetDeclarationsResponseThresholdSectorsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    pub source: String,
}

impl EuSmeThresholdGetDeclarationsResponseThreshold {
    pub fn builder() -> EuSmeThresholdGetDeclarationsResponseThresholdBuilder {
        <EuSmeThresholdGetDeclarationsResponseThresholdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdGetDeclarationsResponseThresholdBuilder {
    currency: Option<String>,
    national_threshold: Option<String>,
    sectors: Option<Vec<EuSmeThresholdGetDeclarationsResponseThresholdSectorsItem>>,
    note: Option<String>,
    source: Option<String>,
}

impl EuSmeThresholdGetDeclarationsResponseThresholdBuilder {
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
        value: Vec<EuSmeThresholdGetDeclarationsResponseThresholdSectorsItem>,
    ) -> Self {
        self.sectors = Some(value);
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

    /// Consumes the builder and constructs a [`EuSmeThresholdGetDeclarationsResponseThreshold`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](EuSmeThresholdGetDeclarationsResponseThresholdBuilder::currency)
    /// - [`source`](EuSmeThresholdGetDeclarationsResponseThresholdBuilder::source)
    pub fn build(self) -> Result<EuSmeThresholdGetDeclarationsResponseThreshold, BuildError> {
        Ok(EuSmeThresholdGetDeclarationsResponseThreshold {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            national_threshold: self.national_threshold,
            sectors: self.sectors,
            note: self.note,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
