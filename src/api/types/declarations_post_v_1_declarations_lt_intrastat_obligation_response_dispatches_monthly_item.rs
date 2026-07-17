pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItem {
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub cumulative: String,
}

impl PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItem {
    pub fn builder() -> PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItemBuilder
    {
        <PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItemBuilder {
    month: Option<i64>,
    value: Option<String>,
    cumulative: Option<String>,
}

impl PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`month`](PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItemBuilder::month)
    /// - [`value`](PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItemBuilder::value)
    /// - [`cumulative`](PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItemBuilder::cumulative)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItem, BuildError>
    {
        Ok(
            PostV1DeclarationsLtIntrastatObligationResponseDispatchesMonthlyItem {
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
