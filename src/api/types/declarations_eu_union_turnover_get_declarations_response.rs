pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuUnionTurnoverGetDeclarationsResponse {
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
    pub current_year: EuUnionTurnoverGetDeclarationsResponseCurrentYear,
    #[serde(rename = "previousYear")]
    #[serde(default)]
    pub previous_year: EuUnionTurnoverGetDeclarationsResponsePreviousYear,
    pub status: EuUnionTurnoverGetDeclarationsResponseStatus,
    #[serde(rename = "headroomAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headroom_amount: Option<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl EuUnionTurnoverGetDeclarationsResponse {
    pub fn builder() -> EuUnionTurnoverGetDeclarationsResponseBuilder {
        <EuUnionTurnoverGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuUnionTurnoverGetDeclarationsResponseBuilder {
    cap_eur: Option<String>,
    currency: Option<String>,
    is_vat_payer: Option<bool>,
    current_year: Option<EuUnionTurnoverGetDeclarationsResponseCurrentYear>,
    previous_year: Option<EuUnionTurnoverGetDeclarationsResponsePreviousYear>,
    status: Option<EuUnionTurnoverGetDeclarationsResponseStatus>,
    headroom_amount: Option<String>,
    warnings: Option<Vec<String>>,
}

impl EuUnionTurnoverGetDeclarationsResponseBuilder {
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
        value: EuUnionTurnoverGetDeclarationsResponseCurrentYear,
    ) -> Self {
        self.current_year = Some(value);
        self
    }

    pub fn previous_year(
        mut self,
        value: EuUnionTurnoverGetDeclarationsResponsePreviousYear,
    ) -> Self {
        self.previous_year = Some(value);
        self
    }

    pub fn status(mut self, value: EuUnionTurnoverGetDeclarationsResponseStatus) -> Self {
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

    /// Consumes the builder and constructs a [`EuUnionTurnoverGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cap_eur`](EuUnionTurnoverGetDeclarationsResponseBuilder::cap_eur)
    /// - [`currency`](EuUnionTurnoverGetDeclarationsResponseBuilder::currency)
    /// - [`is_vat_payer`](EuUnionTurnoverGetDeclarationsResponseBuilder::is_vat_payer)
    /// - [`current_year`](EuUnionTurnoverGetDeclarationsResponseBuilder::current_year)
    /// - [`previous_year`](EuUnionTurnoverGetDeclarationsResponseBuilder::previous_year)
    /// - [`status`](EuUnionTurnoverGetDeclarationsResponseBuilder::status)
    /// - [`warnings`](EuUnionTurnoverGetDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<EuUnionTurnoverGetDeclarationsResponse, BuildError> {
        Ok(EuUnionTurnoverGetDeclarationsResponse {
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
