pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuDistanceSalesThresholdGetResponse {
    #[serde(rename = "thresholdEur")]
    #[serde(default)]
    pub threshold_eur: String,
    #[serde(rename = "homeCountryCode")]
    #[serde(default)]
    pub home_country_code: String,
    #[serde(rename = "currentYear")]
    #[serde(default)]
    pub current_year: PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYear,
    #[serde(rename = "precedingYear")]
    #[serde(default)]
    pub preceding_year: PostV1DeclarationsEuDistanceSalesThresholdGetResponsePrecedingYear,
    #[serde(rename = "belowThreshold")]
    #[serde(default)]
    pub below_threshold: bool,
    #[serde(rename = "headroomAmount")]
    #[serde(default)]
    pub headroom_amount: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsEuDistanceSalesThresholdGetResponse {
    pub fn builder() -> PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder {
        <PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder {
    threshold_eur: Option<String>,
    home_country_code: Option<String>,
    current_year: Option<PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYear>,
    preceding_year: Option<PostV1DeclarationsEuDistanceSalesThresholdGetResponsePrecedingYear>,
    below_threshold: Option<bool>,
    headroom_amount: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder {
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
        value: PostV1DeclarationsEuDistanceSalesThresholdGetResponseCurrentYear,
    ) -> Self {
        self.current_year = Some(value);
        self
    }

    pub fn preceding_year(
        mut self,
        value: PostV1DeclarationsEuDistanceSalesThresholdGetResponsePrecedingYear,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuDistanceSalesThresholdGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`threshold_eur`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder::threshold_eur)
    /// - [`home_country_code`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder::home_country_code)
    /// - [`current_year`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder::current_year)
    /// - [`preceding_year`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder::preceding_year)
    /// - [`below_threshold`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder::below_threshold)
    /// - [`headroom_amount`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder::headroom_amount)
    /// - [`warnings`](PostV1DeclarationsEuDistanceSalesThresholdGetResponseBuilder::warnings)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuDistanceSalesThresholdGetResponse, BuildError> {
        Ok(PostV1DeclarationsEuDistanceSalesThresholdGetResponse {
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
