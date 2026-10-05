pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseMembershipsItem {
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub since: String,
}

impl ExportAccountResponseMembershipsItem {
    pub fn builder() -> ExportAccountResponseMembershipsItemBuilder {
        <ExportAccountResponseMembershipsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseMembershipsItemBuilder {
    company_id: Option<String>,
    company_name: Option<String>,
    role: Option<String>,
    since: Option<String>,
}

impl ExportAccountResponseMembershipsItemBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.since = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExportAccountResponseMembershipsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](ExportAccountResponseMembershipsItemBuilder::company_id)
    /// - [`company_name`](ExportAccountResponseMembershipsItemBuilder::company_name)
    /// - [`role`](ExportAccountResponseMembershipsItemBuilder::role)
    /// - [`since`](ExportAccountResponseMembershipsItemBuilder::since)
    pub fn build(self) -> Result<ExportAccountResponseMembershipsItem, BuildError> {
        Ok(ExportAccountResponseMembershipsItem {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            since: self
                .since
                .ok_or_else(|| BuildError::missing_field("since"))?,
        })
    }
}
