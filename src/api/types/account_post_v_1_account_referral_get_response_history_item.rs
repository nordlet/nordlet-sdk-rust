pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountReferralGetResponseHistoryItem {
    #[serde(default)]
    pub points: i64,
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1AccountReferralGetResponseHistoryItem {
    pub fn builder() -> PostV1AccountReferralGetResponseHistoryItemBuilder {
        <PostV1AccountReferralGetResponseHistoryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountReferralGetResponseHistoryItemBuilder {
    points: Option<i64>,
    reason: Option<String>,
    created_at: Option<String>,
}

impl PostV1AccountReferralGetResponseHistoryItemBuilder {
    pub fn points(mut self, value: i64) -> Self {
        self.points = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountReferralGetResponseHistoryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`points`](PostV1AccountReferralGetResponseHistoryItemBuilder::points)
    /// - [`reason`](PostV1AccountReferralGetResponseHistoryItemBuilder::reason)
    /// - [`created_at`](PostV1AccountReferralGetResponseHistoryItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1AccountReferralGetResponseHistoryItem, BuildError> {
        Ok(PostV1AccountReferralGetResponseHistoryItem {
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
