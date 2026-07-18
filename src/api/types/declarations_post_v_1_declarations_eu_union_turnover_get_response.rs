pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuUnionTurnoverGetResponse {
    #[serde(rename = "capEur")]
    #[serde(default)]
    pub cap_eur: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "isVatPayer")]
    #[serde(default)]
    pub is_vat_payer: bool,
    #[serde(rename = "currentYear")]
    #[serde(default)]
    pub current_year: PostV1DeclarationsEuUnionTurnoverGetResponseCurrentYear,
    #[serde(rename = "previousYear")]
    #[serde(default)]
    pub previous_year: PostV1DeclarationsEuUnionTurnoverGetResponsePreviousYear,
    pub status: PostV1DeclarationsEuUnionTurnoverGetResponseStatus,
    #[serde(rename = "headroomAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headroom_amount: Option<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsEuUnionTurnoverGetResponse {
    pub fn builder() -> PostV1DeclarationsEuUnionTurnoverGetResponseBuilder {
        <PostV1DeclarationsEuUnionTurnoverGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuUnionTurnoverGetResponseBuilder {
    cap_eur: Option<String>,
    currency: Option<String>,
    is_vat_payer: Option<bool>,
    current_year: Option<PostV1DeclarationsEuUnionTurnoverGetResponseCurrentYear>,
    previous_year: Option<PostV1DeclarationsEuUnionTurnoverGetResponsePreviousYear>,
    status: Option<PostV1DeclarationsEuUnionTurnoverGetResponseStatus>,
    headroom_amount: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsEuUnionTurnoverGetResponseBuilder {
    pub fn cap_eur(mut self, value: impl Into<String>) -> Self {
        self.cap_eur = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn is_vat_payer(mut self, value: bool) -> Self {
        self.is_vat_payer = Some(value);
        self
    }

    pub fn current_year(
        mut self,
        value: PostV1DeclarationsEuUnionTurnoverGetResponseCurrentYear,
    ) -> Self {
        self.current_year = Some(value);
        self
    }

    pub fn previous_year(
        mut self,
        value: PostV1DeclarationsEuUnionTurnoverGetResponsePreviousYear,
    ) -> Self {
        self.previous_year = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1DeclarationsEuUnionTurnoverGetResponseStatus) -> Self {
        self.status = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuUnionTurnoverGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cap_eur`](PostV1DeclarationsEuUnionTurnoverGetResponseBuilder::cap_eur)
    /// - [`currency`](PostV1DeclarationsEuUnionTurnoverGetResponseBuilder::currency)
    /// - [`is_vat_payer`](PostV1DeclarationsEuUnionTurnoverGetResponseBuilder::is_vat_payer)
    /// - [`current_year`](PostV1DeclarationsEuUnionTurnoverGetResponseBuilder::current_year)
    /// - [`previous_year`](PostV1DeclarationsEuUnionTurnoverGetResponseBuilder::previous_year)
    /// - [`status`](PostV1DeclarationsEuUnionTurnoverGetResponseBuilder::status)
    /// - [`warnings`](PostV1DeclarationsEuUnionTurnoverGetResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1DeclarationsEuUnionTurnoverGetResponse, BuildError> {
        Ok(PostV1DeclarationsEuUnionTurnoverGetResponse {
            cap_eur: self
                .cap_eur
                .ok_or_else(|| BuildError::missing_field("cap_eur"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            is_vat_payer: self
                .is_vat_payer
                .ok_or_else(|| BuildError::missing_field("is_vat_payer"))?,
            current_year: self
                .current_year
                .ok_or_else(|| BuildError::missing_field("current_year"))?,
            previous_year: self
                .previous_year
                .ok_or_else(|| BuildError::missing_field("previous_year"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            headroom_amount: self.headroom_amount,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
