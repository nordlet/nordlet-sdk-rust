pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CompaniesContextAccountResponseFilingsItemMissingSettings {
    pub operation: CompaniesContextAccountResponseFilingsItemMissingSettingsOperation,
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub keys: Vec<String>,
    #[serde(default)]
    pub call: String,
}

impl CompaniesContextAccountResponseFilingsItemMissingSettings {
    pub fn builder() -> CompaniesContextAccountResponseFilingsItemMissingSettingsBuilder {
        <CompaniesContextAccountResponseFilingsItemMissingSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesContextAccountResponseFilingsItemMissingSettingsBuilder {
    operation: Option<CompaniesContextAccountResponseFilingsItemMissingSettingsOperation>,
    system: Option<String>,
    keys: Option<Vec<String>>,
    call: Option<String>,
}

impl CompaniesContextAccountResponseFilingsItemMissingSettingsBuilder {
    pub fn operation(
        mut self,
        value: CompaniesContextAccountResponseFilingsItemMissingSettingsOperation,
    ) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn keys(mut self, value: Vec<String>) -> Self {
        self.keys = Some(value);
        self
    }

    pub fn call(mut self, value: impl Into<String>) -> Self {
        self.call = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompaniesContextAccountResponseFilingsItemMissingSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operation`](CompaniesContextAccountResponseFilingsItemMissingSettingsBuilder::operation)
    /// - [`system`](CompaniesContextAccountResponseFilingsItemMissingSettingsBuilder::system)
    /// - [`keys`](CompaniesContextAccountResponseFilingsItemMissingSettingsBuilder::keys)
    /// - [`call`](CompaniesContextAccountResponseFilingsItemMissingSettingsBuilder::call)
    pub fn build(
        self,
    ) -> Result<CompaniesContextAccountResponseFilingsItemMissingSettings, BuildError> {
        Ok(CompaniesContextAccountResponseFilingsItemMissingSettings {
            operation: self
                .operation
                .ok_or_else(|| BuildError::missing_field("operation"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            keys: self.keys.ok_or_else(|| BuildError::missing_field("keys"))?,
            call: self.call.ok_or_else(|| BuildError::missing_field("call"))?,
        })
    }
}
