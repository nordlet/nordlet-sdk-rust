pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProjectsRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl CreateProjectsRequest {
    pub fn builder() -> CreateProjectsRequestBuilder {
        <CreateProjectsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProjectsRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    partner_id: Option<String>,
    notes: Option<String>,
}

impl CreateProjectsRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProjectsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CreateProjectsRequestBuilder::code)
    /// - [`name`](CreateProjectsRequestBuilder::name)
    pub fn build(self) -> Result<CreateProjectsRequest, BuildError> {
        Ok(CreateProjectsRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            partner_id: self.partner_id,
            notes: self.notes,
        })
    }
}
