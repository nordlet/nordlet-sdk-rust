pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsCommissionRequest {
    #[serde(rename = "lineId")]
    #[serde(default)]
    pub line_id: String,
    #[serde(rename = "commissionPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_percent: Option<String>,
    #[serde(rename = "commissionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_amount: Option<String>,
}

impl PostV1BankSettlementsCommissionRequest {
    pub fn builder() -> PostV1BankSettlementsCommissionRequestBuilder {
        <PostV1BankSettlementsCommissionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsCommissionRequestBuilder {
    line_id: Option<String>,
    commission_percent: Option<String>,
    commission_amount: Option<String>,
}

impl PostV1BankSettlementsCommissionRequestBuilder {
    pub fn line_id(mut self, value: impl Into<String>) -> Self {
        self.line_id = Some(value.into());
        self
    }

    pub fn commission_percent(mut self, value: impl Into<String>) -> Self {
        self.commission_percent = Some(value.into());
        self
    }

    pub fn commission_amount(mut self, value: impl Into<String>) -> Self {
        self.commission_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsCommissionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line_id`](PostV1BankSettlementsCommissionRequestBuilder::line_id)
    pub fn build(self) -> Result<PostV1BankSettlementsCommissionRequest, BuildError> {
        Ok(PostV1BankSettlementsCommissionRequest {
            line_id: self
                .line_id
                .ok_or_else(|| BuildError::missing_field("line_id"))?,
            commission_percent: self.commission_percent,
            commission_amount: self.commission_amount,
        })
    }
}
