pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "nationalThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_threshold: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectors:
        Option<Vec<PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItem>>,
    #[serde(rename = "intraEuAcquisitionsTrigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_eu_acquisitions_trigger: Option<
        PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTrigger,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItem {
    pub fn builder() -> PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemBuilder {
        <PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemBuilder {
    country_code: Option<String>,
    currency: Option<String>,
    national_threshold: Option<String>,
    sectors: Option<Vec<PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItem>>,
    intra_eu_acquisitions_trigger: Option<
        PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTrigger,
    >,
    note: Option<String>,
    source: Option<String>,
}

impl PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemBuilder {
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
        value: Vec<PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemSectorsItem>,
    ) -> Self {
        self.sectors = Some(value);
        self
    }

    pub fn intra_eu_acquisitions_trigger(
        mut self,
        value: PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemIntraEuAcquisitionsTrigger,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemBuilder::country_code)
    /// - [`currency`](PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemBuilder::currency)
    /// - [`source`](PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItemBuilder::source)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItem, BuildError> {
        Ok(
            PostV1DeclarationsEuSmeThresholdsListResponseThresholdsItem {
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
            },
        )
    }
}
