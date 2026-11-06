pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesConvertToLiveAccountResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "isSandbox")]
    #[serde(default)]
    pub is_sandbox: bool,
}

impl CompaniesConvertToLiveAccountResponse {
    pub fn builder() -> CompaniesConvertToLiveAccountResponseBuilder {
        <CompaniesConvertToLiveAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesConvertToLiveAccountResponseBuilder {
    id: Option<String>,
    is_sandbox: Option<bool>,
}

impl CompaniesConvertToLiveAccountResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_sandbox(mut self, value: bool) -> Self {
        self.is_sandbox = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesConvertToLiveAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompaniesConvertToLiveAccountResponseBuilder::id)
    /// - [`is_sandbox`](CompaniesConvertToLiveAccountResponseBuilder::is_sandbox)
    pub fn build(self) -> Result<CompaniesConvertToLiveAccountResponse, BuildError> {
        Ok(CompaniesConvertToLiveAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_sandbox: self
                .is_sandbox
                .ok_or_else(|| BuildError::missing_field("is_sandbox"))?,
        })
    }
}
