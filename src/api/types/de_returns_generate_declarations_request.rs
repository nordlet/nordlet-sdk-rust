pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnsGenerateDeclarationsRequest {
    #[serde(rename = "ruleKey")]
    pub rule_key: DeReturnsGenerateDeclarationsRequestRuleKey,
    #[serde(default)]
    pub period: String,
}

impl DeReturnsGenerateDeclarationsRequest {
    pub fn builder() -> DeReturnsGenerateDeclarationsRequestBuilder {
        <DeReturnsGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnsGenerateDeclarationsRequestBuilder {
    rule_key: Option<DeReturnsGenerateDeclarationsRequestRuleKey>,
    period: Option<String>,
}

impl DeReturnsGenerateDeclarationsRequestBuilder {
    pub fn rule_key(mut self, value: DeReturnsGenerateDeclarationsRequestRuleKey) -> Self {
        self.rule_key = Some(value);
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnsGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_key`](DeReturnsGenerateDeclarationsRequestBuilder::rule_key)
    /// - [`period`](DeReturnsGenerateDeclarationsRequestBuilder::period)
    pub fn build(self) -> Result<DeReturnsGenerateDeclarationsRequest, BuildError> {
        Ok(DeReturnsGenerateDeclarationsRequest {
            rule_key: self
                .rule_key
                .ok_or_else(|| BuildError::missing_field("rule_key"))?,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
        })
    }
}
