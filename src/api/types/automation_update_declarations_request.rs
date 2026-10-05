pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationUpdateDeclarationsRequest {
    #[serde(rename = "ruleKey")]
    #[serde(default)]
    pub rule_key: String,
    #[serde(default)]
    pub enabled: bool,
}

impl AutomationUpdateDeclarationsRequest {
    pub fn builder() -> AutomationUpdateDeclarationsRequestBuilder {
        <AutomationUpdateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationUpdateDeclarationsRequestBuilder {
    rule_key: Option<String>,
    enabled: Option<bool>,
}

impl AutomationUpdateDeclarationsRequestBuilder {
    pub fn rule_key(mut self, value: impl Into<String>) -> Self {
        self.rule_key = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationUpdateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_key`](AutomationUpdateDeclarationsRequestBuilder::rule_key)
    /// - [`enabled`](AutomationUpdateDeclarationsRequestBuilder::enabled)
    pub fn build(self) -> Result<AutomationUpdateDeclarationsRequest, BuildError> {
        Ok(AutomationUpdateDeclarationsRequest {
            rule_key: self
                .rule_key
                .ok_or_else(|| BuildError::missing_field("rule_key"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
