pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesCreateAccountResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "isSandbox")]
    #[serde(default)]
    pub is_sandbox: bool,
}

impl CompaniesCreateAccountResponse {
    pub fn builder() -> CompaniesCreateAccountResponseBuilder {
        <CompaniesCreateAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesCreateAccountResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    role: Option<String>,
    is_sandbox: Option<bool>,
}

impl CompaniesCreateAccountResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn is_sandbox(mut self, value: bool) -> Self {
        self.is_sandbox = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesCreateAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompaniesCreateAccountResponseBuilder::id)
    /// - [`name`](CompaniesCreateAccountResponseBuilder::name)
    /// - [`role`](CompaniesCreateAccountResponseBuilder::role)
    /// - [`is_sandbox`](CompaniesCreateAccountResponseBuilder::is_sandbox)
    pub fn build(self) -> Result<CompaniesCreateAccountResponse, BuildError> {
        Ok(CompaniesCreateAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            is_sandbox: self
                .is_sandbox
                .ok_or_else(|| BuildError::missing_field("is_sandbox"))?,
        })
    }
}
