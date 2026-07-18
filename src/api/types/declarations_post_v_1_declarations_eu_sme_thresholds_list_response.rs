pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdsListResponse {
    #[serde(rename = "nationalCapEur")]
    #[serde(default)]
    pub national_cap_eur: String,
    #[serde(rename = "unionTurnoverCapEur")]
    #[serde(default)]
    pub union_turnover_cap_eur: String,
    #[serde(default)]
    pub thresholds: Vec<PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItem>,
}

impl PostV1DeclarationsEuSmeThresholdsListResponse {
    pub fn builder() -> PostV1DeclarationsEuSmeThresholdsListResponseBuilder {
        <PostV1DeclarationsEuSmeThresholdsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdsListResponseBuilder {
    national_cap_eur: Option<String>,
    union_turnover_cap_eur: Option<String>,
    thresholds: Option<Vec<PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItem>>,
}

impl PostV1DeclarationsEuSmeThresholdsListResponseBuilder {
    pub fn national_cap_eur(mut self, value: impl Into<String>) -> Self {
        self.national_cap_eur = Some(value.into());
        self
    }

    pub fn union_turnover_cap_eur(mut self, value: impl Into<String>) -> Self {
        self.union_turnover_cap_eur = Some(value.into());
        self
    }

    pub fn thresholds(
        mut self,
        value: Vec<PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItem>,
    ) -> Self {
        self.thresholds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`national_cap_eur`](PostV1DeclarationsEuSmeThresholdsListResponseBuilder::national_cap_eur)
    /// - [`union_turnover_cap_eur`](PostV1DeclarationsEuSmeThresholdsListResponseBuilder::union_turnover_cap_eur)
    /// - [`thresholds`](PostV1DeclarationsEuSmeThresholdsListResponseBuilder::thresholds)
    pub fn build(self) -> Result<PostV1DeclarationsEuSmeThresholdsListResponse, BuildError> {
        Ok(PostV1DeclarationsEuSmeThresholdsListResponse {
            national_cap_eur: self
                .national_cap_eur
                .ok_or_else(|| BuildError::missing_field("national_cap_eur"))?,
            union_turnover_cap_eur: self
                .union_turnover_cap_eur
                .ok_or_else(|| BuildError::missing_field("union_turnover_cap_eur"))?,
            thresholds: self
                .thresholds
                .ok_or_else(|| BuildError::missing_field("thresholds"))?,
        })
    }
}
