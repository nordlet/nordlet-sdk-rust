pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIntrastatObligationDeclarationsResponseDispatches {
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
    pub monthly: Vec<LtIntrastatObligationDeclarationsResponseDispatchesMonthlyItem>,
}

impl LtIntrastatObligationDeclarationsResponseDispatches {
    pub fn builder() -> LtIntrastatObligationDeclarationsResponseDispatchesBuilder {
        <LtIntrastatObligationDeclarationsResponseDispatchesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIntrastatObligationDeclarationsResponseDispatchesBuilder {
    previous_year_value: Option<String>,
    obligated_from_month: Option<i64>,
    statistical_value_required: Option<bool>,
    monthly: Option<Vec<LtIntrastatObligationDeclarationsResponseDispatchesMonthlyItem>>,
}

impl LtIntrastatObligationDeclarationsResponseDispatchesBuilder {
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
        value: Vec<LtIntrastatObligationDeclarationsResponseDispatchesMonthlyItem>,
    ) -> Self {
        self.monthly = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIntrastatObligationDeclarationsResponseDispatches`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_year_value`](LtIntrastatObligationDeclarationsResponseDispatchesBuilder::previous_year_value)
    /// - [`statistical_value_required`](LtIntrastatObligationDeclarationsResponseDispatchesBuilder::statistical_value_required)
    /// - [`monthly`](LtIntrastatObligationDeclarationsResponseDispatchesBuilder::monthly)
    pub fn build(self) -> Result<LtIntrastatObligationDeclarationsResponseDispatches, BuildError> {
        Ok(LtIntrastatObligationDeclarationsResponseDispatches {
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
