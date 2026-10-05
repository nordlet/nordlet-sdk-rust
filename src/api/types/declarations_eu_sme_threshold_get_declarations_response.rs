pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdGetDeclarationsResponse {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "isVatPayer")]
    #[serde(default)]
    pub is_vat_payer: bool,
    #[serde(rename = "baseCurrency")]
    #[serde(default)]
    pub base_currency: String,
    #[serde(default)]
    pub year: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<EuSmeThresholdGetDeclarationsResponseThreshold>,
    #[serde(default)]
    pub turnover: EuSmeThresholdGetDeclarationsResponseTurnover,
    #[serde(rename = "precedingTurnover")]
    #[serde(default)]
    pub preceding_turnover: EuSmeThresholdGetDeclarationsResponsePrecedingTurnover,
    pub status: EuSmeThresholdGetDeclarationsResponseStatus,
    #[serde(rename = "headroomAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headroom_amount: Option<String>,
    #[serde(rename = "intraEu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_eu: Option<EuSmeThresholdGetDeclarationsResponseIntraEu>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl EuSmeThresholdGetDeclarationsResponse {
    pub fn builder() -> EuSmeThresholdGetDeclarationsResponseBuilder {
        <EuSmeThresholdGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdGetDeclarationsResponseBuilder {
    country_code: Option<String>,
    is_vat_payer: Option<bool>,
    base_currency: Option<String>,
    year: Option<i64>,
    threshold: Option<EuSmeThresholdGetDeclarationsResponseThreshold>,
    turnover: Option<EuSmeThresholdGetDeclarationsResponseTurnover>,
    preceding_turnover: Option<EuSmeThresholdGetDeclarationsResponsePrecedingTurnover>,
    status: Option<EuSmeThresholdGetDeclarationsResponseStatus>,
    headroom_amount: Option<String>,
    intra_eu: Option<EuSmeThresholdGetDeclarationsResponseIntraEu>,
    warnings: Option<Vec<String>>,
}

impl EuSmeThresholdGetDeclarationsResponseBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn is_vat_payer(mut self, value: bool) -> Self {
        self.is_vat_payer = Some(value);
        self
    }

    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_currency = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn threshold(mut self, value: EuSmeThresholdGetDeclarationsResponseThreshold) -> Self {
        self.threshold = Some(value);
        self
    }

    pub fn turnover(mut self, value: EuSmeThresholdGetDeclarationsResponseTurnover) -> Self {
        self.turnover = Some(value);
        self
    }

    pub fn preceding_turnover(
        mut self,
        value: EuSmeThresholdGetDeclarationsResponsePrecedingTurnover,
    ) -> Self {
        self.preceding_turnover = Some(value);
        self
    }

    pub fn status(mut self, value: EuSmeThresholdGetDeclarationsResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn headroom_amount(mut self, value: impl Into<String>) -> Self {
        self.headroom_amount = Some(value.into());
        self
    }

    pub fn intra_eu(mut self, value: EuSmeThresholdGetDeclarationsResponseIntraEu) -> Self {
        self.intra_eu = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuSmeThresholdGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuSmeThresholdGetDeclarationsResponseBuilder::country_code)
    /// - [`is_vat_payer`](EuSmeThresholdGetDeclarationsResponseBuilder::is_vat_payer)
    /// - [`base_currency`](EuSmeThresholdGetDeclarationsResponseBuilder::base_currency)
    /// - [`year`](EuSmeThresholdGetDeclarationsResponseBuilder::year)
    /// - [`turnover`](EuSmeThresholdGetDeclarationsResponseBuilder::turnover)
    /// - [`preceding_turnover`](EuSmeThresholdGetDeclarationsResponseBuilder::preceding_turnover)
    /// - [`status`](EuSmeThresholdGetDeclarationsResponseBuilder::status)
    /// - [`warnings`](EuSmeThresholdGetDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<EuSmeThresholdGetDeclarationsResponse, BuildError> {
        Ok(EuSmeThresholdGetDeclarationsResponse {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            is_vat_payer: self
                .is_vat_payer
                .ok_or_else(|| BuildError::missing_field("is_vat_payer"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            threshold: self.threshold,
            turnover: self
                .turnover
                .ok_or_else(|| BuildError::missing_field("turnover"))?,
            preceding_turnover: self
                .preceding_turnover
                .ok_or_else(|| BuildError::missing_field("preceding_turnover"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            headroom_amount: self.headroom_amount,
            intra_eu: self.intra_eu,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
