pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutomationListDeclarationsResponseRowsItem {
    #[serde(rename = "ruleKey")]
    #[serde(default)]
    pub rule_key: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub applies: bool,
    #[serde(default)]
    pub configured: bool,
    pub certificate: AutomationListDeclarationsResponseRowsItemCertificate,
    pub environment: AutomationListDeclarationsResponseRowsItemEnvironment,
}

impl AutomationListDeclarationsResponseRowsItem {
    pub fn builder() -> AutomationListDeclarationsResponseRowsItemBuilder {
        <AutomationListDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationListDeclarationsResponseRowsItemBuilder {
    rule_key: Option<String>,
    title: Option<String>,
    country: Option<String>,
    system: Option<String>,
    enabled: Option<bool>,
    applies: Option<bool>,
    configured: Option<bool>,
    certificate: Option<AutomationListDeclarationsResponseRowsItemCertificate>,
    environment: Option<AutomationListDeclarationsResponseRowsItemEnvironment>,
}

impl AutomationListDeclarationsResponseRowsItemBuilder {
    pub fn rule_key(mut self, value: impl Into<String>) -> Self {
        self.rule_key = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn applies(mut self, value: bool) -> Self {
        self.applies = Some(value);
        self
    }

    pub fn configured(mut self, value: bool) -> Self {
        self.configured = Some(value);
        self
    }

    pub fn certificate(
        mut self,
        value: AutomationListDeclarationsResponseRowsItemCertificate,
    ) -> Self {
        self.certificate = Some(value);
        self
    }

    pub fn environment(
        mut self,
        value: AutomationListDeclarationsResponseRowsItemEnvironment,
    ) -> Self {
        self.environment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationListDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_key`](AutomationListDeclarationsResponseRowsItemBuilder::rule_key)
    /// - [`title`](AutomationListDeclarationsResponseRowsItemBuilder::title)
    /// - [`country`](AutomationListDeclarationsResponseRowsItemBuilder::country)
    /// - [`system`](AutomationListDeclarationsResponseRowsItemBuilder::system)
    /// - [`enabled`](AutomationListDeclarationsResponseRowsItemBuilder::enabled)
    /// - [`applies`](AutomationListDeclarationsResponseRowsItemBuilder::applies)
    /// - [`configured`](AutomationListDeclarationsResponseRowsItemBuilder::configured)
    /// - [`certificate`](AutomationListDeclarationsResponseRowsItemBuilder::certificate)
    /// - [`environment`](AutomationListDeclarationsResponseRowsItemBuilder::environment)
    pub fn build(self) -> Result<AutomationListDeclarationsResponseRowsItem, BuildError> {
        Ok(AutomationListDeclarationsResponseRowsItem {
            rule_key: self
                .rule_key
                .ok_or_else(|| BuildError::missing_field("rule_key"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            applies: self
                .applies
                .ok_or_else(|| BuildError::missing_field("applies"))?,
            configured: self
                .configured
                .ok_or_else(|| BuildError::missing_field("configured"))?,
            certificate: self
                .certificate
                .ok_or_else(|| BuildError::missing_field("certificate"))?,
            environment: self
                .environment
                .ok_or_else(|| BuildError::missing_field("environment"))?,
        })
    }
}
