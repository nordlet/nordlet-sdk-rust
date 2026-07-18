pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdGetResponse {
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
    pub threshold: Option<PostV1DeclarationsEuSmeThresholdGetResponseThreshold>,
    #[serde(default)]
    pub turnover: PostV1DeclarationsEuSmeThresholdGetResponseTurnover,
    #[serde(rename = "precedingTurnover")]
    #[serde(default)]
    pub preceding_turnover: PostV1DeclarationsEuSmeThresholdGetResponsePrecedingTurnover,
    pub status: PostV1DeclarationsEuSmeThresholdGetResponseStatus,
    #[serde(rename = "headroomAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headroom_amount: Option<String>,
    #[serde(rename = "intraEu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_eu: Option<PostV1DeclarationsEuSmeThresholdGetResponseIntraEu>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsEuSmeThresholdGetResponse {
    pub fn builder() -> PostV1DeclarationsEuSmeThresholdGetResponseBuilder {
        <PostV1DeclarationsEuSmeThresholdGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdGetResponseBuilder {
    country_code: Option<String>,
    is_vat_payer: Option<bool>,
    base_currency: Option<String>,
    year: Option<i64>,
    threshold: Option<PostV1DeclarationsEuSmeThresholdGetResponseThreshold>,
    turnover: Option<PostV1DeclarationsEuSmeThresholdGetResponseTurnover>,
    preceding_turnover: Option<PostV1DeclarationsEuSmeThresholdGetResponsePrecedingTurnover>,
    status: Option<PostV1DeclarationsEuSmeThresholdGetResponseStatus>,
    headroom_amount: Option<String>,
    intra_eu: Option<PostV1DeclarationsEuSmeThresholdGetResponseIntraEu>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsEuSmeThresholdGetResponseBuilder {
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

    pub fn threshold(
        mut self,
        value: PostV1DeclarationsEuSmeThresholdGetResponseThreshold,
    ) -> Self {
        self.threshold = Some(value);
        self
    }

    pub fn turnover(mut self, value: PostV1DeclarationsEuSmeThresholdGetResponseTurnover) -> Self {
        self.turnover = Some(value);
        self
    }

    pub fn preceding_turnover(
        mut self,
        value: PostV1DeclarationsEuSmeThresholdGetResponsePrecedingTurnover,
    ) -> Self {
        self.preceding_turnover = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1DeclarationsEuSmeThresholdGetResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn headroom_amount(mut self, value: impl Into<String>) -> Self {
        self.headroom_amount = Some(value.into());
        self
    }

    pub fn intra_eu(mut self, value: PostV1DeclarationsEuSmeThresholdGetResponseIntraEu) -> Self {
        self.intra_eu = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsEuSmeThresholdGetResponseBuilder::country_code)
    /// - [`is_vat_payer`](PostV1DeclarationsEuSmeThresholdGetResponseBuilder::is_vat_payer)
    /// - [`base_currency`](PostV1DeclarationsEuSmeThresholdGetResponseBuilder::base_currency)
    /// - [`year`](PostV1DeclarationsEuSmeThresholdGetResponseBuilder::year)
    /// - [`turnover`](PostV1DeclarationsEuSmeThresholdGetResponseBuilder::turnover)
    /// - [`preceding_turnover`](PostV1DeclarationsEuSmeThresholdGetResponseBuilder::preceding_turnover)
    /// - [`status`](PostV1DeclarationsEuSmeThresholdGetResponseBuilder::status)
    /// - [`warnings`](PostV1DeclarationsEuSmeThresholdGetResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1DeclarationsEuSmeThresholdGetResponse, BuildError> {
        Ok(PostV1DeclarationsEuSmeThresholdGetResponse {
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
