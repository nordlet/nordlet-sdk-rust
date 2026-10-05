pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferralGetAccountResponseHistoryItem {
    #[serde(default)]
    pub points: i64,
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ReferralGetAccountResponseHistoryItem {
    pub fn builder() -> ReferralGetAccountResponseHistoryItemBuilder {
        <ReferralGetAccountResponseHistoryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferralGetAccountResponseHistoryItemBuilder {
    points: Option<i64>,
    reason: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ReferralGetAccountResponseHistoryItemBuilder {
    pub fn points(mut self, value: i64) -> Self {
        self.points = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferralGetAccountResponseHistoryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`points`](ReferralGetAccountResponseHistoryItemBuilder::points)
    /// - [`reason`](ReferralGetAccountResponseHistoryItemBuilder::reason)
    /// - [`created_at`](ReferralGetAccountResponseHistoryItemBuilder::created_at)
    pub fn build(self) -> Result<ReferralGetAccountResponseHistoryItem, BuildError> {
        Ok(ReferralGetAccountResponseHistoryItem {
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
