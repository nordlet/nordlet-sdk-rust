pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseReferralPointsItem {
    #[serde(default)]
    pub points: i64,
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ExportAccountResponseReferralPointsItem {
    pub fn builder() -> ExportAccountResponseReferralPointsItemBuilder {
        <ExportAccountResponseReferralPointsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseReferralPointsItemBuilder {
    points: Option<i64>,
    reason: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ExportAccountResponseReferralPointsItemBuilder {
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

    /// Consumes the builder and constructs a [`ExportAccountResponseReferralPointsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`points`](ExportAccountResponseReferralPointsItemBuilder::points)
    /// - [`reason`](ExportAccountResponseReferralPointsItemBuilder::reason)
    /// - [`created_at`](ExportAccountResponseReferralPointsItemBuilder::created_at)
    pub fn build(self) -> Result<ExportAccountResponseReferralPointsItem, BuildError> {
        Ok(ExportAccountResponseReferralPointsItem {
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
