pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeCt1GenerateDeclarationsResponseAccounts {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xhtml: String,
}

impl IeCt1GenerateDeclarationsResponseAccounts {
    pub fn builder() -> IeCt1GenerateDeclarationsResponseAccountsBuilder {
        <IeCt1GenerateDeclarationsResponseAccountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeCt1GenerateDeclarationsResponseAccountsBuilder {
    file_name: Option<String>,
    xhtml: Option<String>,
}

impl IeCt1GenerateDeclarationsResponseAccountsBuilder {
    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xhtml(mut self, value: impl Into<String>) -> Self {
        self.xhtml = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IeCt1GenerateDeclarationsResponseAccounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_name`](IeCt1GenerateDeclarationsResponseAccountsBuilder::file_name)
    /// - [`xhtml`](IeCt1GenerateDeclarationsResponseAccountsBuilder::xhtml)
    pub fn build(self) -> Result<IeCt1GenerateDeclarationsResponseAccounts, BuildError> {
        Ok(IeCt1GenerateDeclarationsResponseAccounts {
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xhtml: self
                .xhtml
                .ok_or_else(|| BuildError::missing_field("xhtml"))?,
        })
    }
}
