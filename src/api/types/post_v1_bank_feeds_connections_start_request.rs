pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsStartRequest {
    #[serde(rename = "aspspName")]
    #[serde(default)]
    pub aspsp_name: String,
    #[serde(rename = "aspspCountry")]
    #[serde(default)]
    pub aspsp_country: String,
    #[serde(rename = "psuType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psu_type: Option<PostV1BankFeedsConnectionsStartRequestPsuType>,
    #[serde(rename = "redirectUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "validForDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl PostV1BankFeedsConnectionsStartRequest {
    pub fn builder() -> PostV1BankFeedsConnectionsStartRequestBuilder {
        <PostV1BankFeedsConnectionsStartRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsStartRequestBuilder {
    aspsp_name: Option<String>,
    aspsp_country: Option<String>,
    psu_type: Option<PostV1BankFeedsConnectionsStartRequestPsuType>,
    redirect_url: Option<String>,
    valid_for_days: Option<i64>,
    language: Option<String>,
}

impl PostV1BankFeedsConnectionsStartRequestBuilder {
    pub fn aspsp_name(mut self, value: impl Into<String>) -> Self {
        self.aspsp_name = Some(value.into());
        self
    }

    pub fn aspsp_country(mut self, value: impl Into<String>) -> Self {
        self.aspsp_country = Some(value.into());
        self
    }

    pub fn psu_type(mut self, value: PostV1BankFeedsConnectionsStartRequestPsuType) -> Self {
        self.psu_type = Some(value);
        self
    }

    pub fn redirect_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_url = Some(value.into());
        self
    }

    pub fn valid_for_days(mut self, value: i64) -> Self {
        self.valid_for_days = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsStartRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aspsp_name`](PostV1BankFeedsConnectionsStartRequestBuilder::aspsp_name)
    /// - [`aspsp_country`](PostV1BankFeedsConnectionsStartRequestBuilder::aspsp_country)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsStartRequest, BuildError> {
        Ok(PostV1BankFeedsConnectionsStartRequest {
            aspsp_name: self
                .aspsp_name
                .ok_or_else(|| BuildError::missing_field("aspsp_name"))?,
            aspsp_country: self
                .aspsp_country
                .ok_or_else(|| BuildError::missing_field("aspsp_country"))?,
            psu_type: self.psu_type,
            redirect_url: self.redirect_url,
            valid_for_days: self.valid_for_days,
            language: self.language,
        })
    }
}
