pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesContextAccountResponseFilingsItem {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "ruleKey")]
    #[serde(default)]
    pub rule_key: String,
    #[serde(default)]
    pub period: String,
    #[serde(default)]
    pub title: String,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(rename = "canDownload")]
    #[serde(default)]
    pub can_download: bool,
    #[serde(rename = "canSubmit")]
    #[serde(default)]
    pub can_submit: bool,
    #[serde(rename = "missingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_settings: Option<CompaniesContextAccountResponseFilingsItemMissingSettings>,
}

impl CompaniesContextAccountResponseFilingsItem {
    pub fn builder() -> CompaniesContextAccountResponseFilingsItemBuilder {
        <CompaniesContextAccountResponseFilingsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesContextAccountResponseFilingsItemBuilder {
    key: Option<String>,
    rule_key: Option<String>,
    period: Option<String>,
    title: Option<String>,
    due_date: Option<NaiveDate>,
    system: Option<String>,
    can_download: Option<bool>,
    can_submit: Option<bool>,
    missing_settings: Option<CompaniesContextAccountResponseFilingsItemMissingSettings>,
}

impl CompaniesContextAccountResponseFilingsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn rule_key(mut self, value: impl Into<String>) -> Self {
        self.rule_key = Some(value.into());
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn can_download(mut self, value: bool) -> Self {
        self.can_download = Some(value);
        self
    }

    pub fn can_submit(mut self, value: bool) -> Self {
        self.can_submit = Some(value);
        self
    }

    pub fn missing_settings(
        mut self,
        value: CompaniesContextAccountResponseFilingsItemMissingSettings,
    ) -> Self {
        self.missing_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesContextAccountResponseFilingsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](CompaniesContextAccountResponseFilingsItemBuilder::key)
    /// - [`rule_key`](CompaniesContextAccountResponseFilingsItemBuilder::rule_key)
    /// - [`period`](CompaniesContextAccountResponseFilingsItemBuilder::period)
    /// - [`title`](CompaniesContextAccountResponseFilingsItemBuilder::title)
    /// - [`due_date`](CompaniesContextAccountResponseFilingsItemBuilder::due_date)
    /// - [`can_download`](CompaniesContextAccountResponseFilingsItemBuilder::can_download)
    /// - [`can_submit`](CompaniesContextAccountResponseFilingsItemBuilder::can_submit)
    pub fn build(self) -> Result<CompaniesContextAccountResponseFilingsItem, BuildError> {
        Ok(CompaniesContextAccountResponseFilingsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            rule_key: self
                .rule_key
                .ok_or_else(|| BuildError::missing_field("rule_key"))?,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            due_date: self
                .due_date
                .ok_or_else(|| BuildError::missing_field("due_date"))?,
            system: self.system,
            can_download: self
                .can_download
                .ok_or_else(|| BuildError::missing_field("can_download"))?,
            can_submit: self
                .can_submit
                .ok_or_else(|| BuildError::missing_field("can_submit"))?,
            missing_settings: self.missing_settings,
        })
    }
}
