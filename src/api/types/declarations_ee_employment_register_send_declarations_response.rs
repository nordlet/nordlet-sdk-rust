pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EeEmploymentRegisterSendDeclarationsResponse {
    #[serde(default)]
    pub reference: String,
    pub state: EeEmploymentRegisterSendDeclarationsResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "entryDate")]
    #[serde(default)]
    pub entry_date: NaiveDate,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl EeEmploymentRegisterSendDeclarationsResponse {
    pub fn builder() -> EeEmploymentRegisterSendDeclarationsResponseBuilder {
        <EeEmploymentRegisterSendDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EeEmploymentRegisterSendDeclarationsResponseBuilder {
    reference: Option<String>,
    state: Option<EeEmploymentRegisterSendDeclarationsResponseState>,
    detail: Option<String>,
    file_name: Option<String>,
    entry_date: Option<NaiveDate>,
    xml: Option<String>,
    warnings: Option<Vec<String>>,
}

impl EeEmploymentRegisterSendDeclarationsResponseBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn state(mut self, value: EeEmploymentRegisterSendDeclarationsResponseState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn entry_date(mut self, value: NaiveDate) -> Self {
        self.entry_date = Some(value);
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EeEmploymentRegisterSendDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](EeEmploymentRegisterSendDeclarationsResponseBuilder::reference)
    /// - [`state`](EeEmploymentRegisterSendDeclarationsResponseBuilder::state)
    /// - [`file_name`](EeEmploymentRegisterSendDeclarationsResponseBuilder::file_name)
    /// - [`entry_date`](EeEmploymentRegisterSendDeclarationsResponseBuilder::entry_date)
    /// - [`xml`](EeEmploymentRegisterSendDeclarationsResponseBuilder::xml)
    /// - [`warnings`](EeEmploymentRegisterSendDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<EeEmploymentRegisterSendDeclarationsResponse, BuildError> {
        Ok(EeEmploymentRegisterSendDeclarationsResponse {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            detail: self.detail,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            entry_date: self
                .entry_date
                .ok_or_else(|| BuildError::missing_field("entry_date"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
