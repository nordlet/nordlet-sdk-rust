pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDistanceSalesThresholdGetDeclarationsResponse {
    #[serde(rename = "thresholdEur")]
    #[serde(default)]
    pub threshold_eur: String,
    #[serde(rename = "homeCountryCode")]
    #[serde(default)]
    pub home_country_code: String,
    #[serde(rename = "currentYear")]
    #[serde(default)]
    pub current_year: EuDistanceSalesThresholdGetDeclarationsResponseCurrentYear,
    #[serde(rename = "precedingYear")]
    #[serde(default)]
    pub preceding_year: EuDistanceSalesThresholdGetDeclarationsResponsePrecedingYear,
    #[serde(rename = "belowThreshold")]
    #[serde(default)]
    pub below_threshold: bool,
    #[serde(rename = "headroomAmount")]
    #[serde(default)]
    pub headroom_amount: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl EuDistanceSalesThresholdGetDeclarationsResponse {
    pub fn builder() -> EuDistanceSalesThresholdGetDeclarationsResponseBuilder {
        <EuDistanceSalesThresholdGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDistanceSalesThresholdGetDeclarationsResponseBuilder {
    threshold_eur: Option<String>,
    home_country_code: Option<String>,
    current_year: Option<EuDistanceSalesThresholdGetDeclarationsResponseCurrentYear>,
    preceding_year: Option<EuDistanceSalesThresholdGetDeclarationsResponsePrecedingYear>,
    below_threshold: Option<bool>,
    headroom_amount: Option<String>,
    warnings: Option<Vec<String>>,
}

impl EuDistanceSalesThresholdGetDeclarationsResponseBuilder {
    pub fn threshold_eur(mut self, value: impl Into<String>) -> Self {
        self.threshold_eur = Some(value.into());
        self
    }

    pub fn home_country_code(mut self, value: impl Into<String>) -> Self {
        self.home_country_code = Some(value.into());
        self
    }

    pub fn current_year(
        mut self,
        value: EuDistanceSalesThresholdGetDeclarationsResponseCurrentYear,
    ) -> Self {
        self.current_year = Some(value);
        self
    }

    pub fn preceding_year(
        mut self,
        value: EuDistanceSalesThresholdGetDeclarationsResponsePrecedingYear,
    ) -> Self {
        self.preceding_year = Some(value);
        self
    }

    pub fn below_threshold(mut self, value: bool) -> Self {
        self.below_threshold = Some(value);
        self
    }

    pub fn headroom_amount(mut self, value: impl Into<String>) -> Self {
        self.headroom_amount = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuDistanceSalesThresholdGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`threshold_eur`](EuDistanceSalesThresholdGetDeclarationsResponseBuilder::threshold_eur)
    /// - [`home_country_code`](EuDistanceSalesThresholdGetDeclarationsResponseBuilder::home_country_code)
    /// - [`current_year`](EuDistanceSalesThresholdGetDeclarationsResponseBuilder::current_year)
    /// - [`preceding_year`](EuDistanceSalesThresholdGetDeclarationsResponseBuilder::preceding_year)
    /// - [`below_threshold`](EuDistanceSalesThresholdGetDeclarationsResponseBuilder::below_threshold)
    /// - [`headroom_amount`](EuDistanceSalesThresholdGetDeclarationsResponseBuilder::headroom_amount)
    /// - [`warnings`](EuDistanceSalesThresholdGetDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<EuDistanceSalesThresholdGetDeclarationsResponse, BuildError> {
        Ok(EuDistanceSalesThresholdGetDeclarationsResponse {
            threshold_eur: self
                .threshold_eur
                .ok_or_else(|| BuildError::missing_field("threshold_eur"))?,
            home_country_code: self
                .home_country_code
                .ok_or_else(|| BuildError::missing_field("home_country_code"))?,
            current_year: self
                .current_year
                .ok_or_else(|| BuildError::missing_field("current_year"))?,
            preceding_year: self
                .preceding_year
                .ok_or_else(|| BuildError::missing_field("preceding_year"))?,
            below_threshold: self
                .below_threshold
                .ok_or_else(|| BuildError::missing_field("below_threshold"))?,
            headroom_amount: self
                .headroom_amount
                .ok_or_else(|| BuildError::missing_field("headroom_amount"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
