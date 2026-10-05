pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItem {
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub cumulative: String,
}

impl LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItem {
    pub fn builder() -> LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItemBuilder {
        <LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItemBuilder {
    month: Option<i64>,
    value: Option<String>,
    cumulative: Option<String>,
}

impl LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItemBuilder {
    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn cumulative(mut self, value: impl Into<String>) -> Self {
        self.cumulative = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`month`](LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItemBuilder::month)
    /// - [`value`](LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItemBuilder::value)
    /// - [`cumulative`](LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItemBuilder::cumulative)
    pub fn build(
        self,
    ) -> Result<LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItem, BuildError> {
        Ok(
            LtIntrastatObligationDeclarationsResponseArrivalsMonthlyItem {
                month: self
                    .month
                    .ok_or_else(|| BuildError::missing_field("month"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                cumulative: self
                    .cumulative
                    .ok_or_else(|| BuildError::missing_field("cumulative"))?,
            },
        )
    }
}
