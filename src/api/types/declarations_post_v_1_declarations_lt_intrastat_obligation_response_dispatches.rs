pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIntrastatObligationResponseDispatches {
    #[serde(rename = "previousYearValue")]
    #[serde(default)]
    pub previous_year_value: String,
    #[serde(rename = "obligatedFromMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obligated_from_month: Option<i64>,
    #[serde(rename = "statisticalValueRequired")]
    #[serde(default)]
    pub statistical_value_required: bool,
    #[serde(default)]
    pub monthly: Vec<PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItem>,
}

impl PostV1DeclarationsLtIntrastatObligationResponseDispatches {
    pub fn builder() -> PostV1DeclarationsLtIntrastatObligationResponseDispatchesBuilder {
        <PostV1DeclarationsLtIntrastatObligationResponseDispatchesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIntrastatObligationResponseDispatchesBuilder {
    previous_year_value: Option<String>,
    obligated_from_month: Option<i64>,
    statistical_value_required: Option<bool>,
    monthly: Option<Vec<PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItem>>,
}

impl PostV1DeclarationsLtIntrastatObligationResponseDispatchesBuilder {
    pub fn previous_year_value(mut self, value: impl Into<String>) -> Self {
        self.previous_year_value = Some(value.into());
        self
    }

    pub fn obligated_from_month(mut self, value: i64) -> Self {
        self.obligated_from_month = Some(value);
        self
    }

    pub fn statistical_value_required(mut self, value: bool) -> Self {
        self.statistical_value_required = Some(value);
        self
    }

    pub fn monthly(
        mut self,
        value: Vec<PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItem>,
    ) -> Self {
        self.monthly = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIntrastatObligationResponseDispatches`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_year_value`](PostV1DeclarationsLtIntrastatObligationResponseDispatchesBuilder::previous_year_value)
    /// - [`statistical_value_required`](PostV1DeclarationsLtIntrastatObligationResponseDispatchesBuilder::statistical_value_required)
    /// - [`monthly`](PostV1DeclarationsLtIntrastatObligationResponseDispatchesBuilder::monthly)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsLtIntrastatObligationResponseDispatches, BuildError> {
        Ok(PostV1DeclarationsLtIntrastatObligationResponseDispatches {
            previous_year_value: self
                .previous_year_value
                .ok_or_else(|| BuildError::missing_field("previous_year_value"))?,
            obligated_from_month: self.obligated_from_month,
            statistical_value_required: self
                .statistical_value_required
                .ok_or_else(|| BuildError::missing_field("statistical_value_required"))?,
            monthly: self
                .monthly
                .ok_or_else(|| BuildError::missing_field("monthly"))?,
        })
    }
}
