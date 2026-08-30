pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountExportResponseMembershipsItem {
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

impl PostV1AccountExportResponseMembershipsItem {
    pub fn builder() -> PostV1AccountExportResponseMembershipsItemBuilder {
        <PostV1AccountExportResponseMembershipsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountExportResponseMembershipsItemBuilder {
    company_id: Option<String>,
    company_name: Option<String>,
    role: Option<String>,
    since: Option<String>,
}

impl PostV1AccountExportResponseMembershipsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AccountExportResponseMembershipsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](PostV1AccountExportResponseMembershipsItemBuilder::company_id)
    /// - [`company_name`](PostV1AccountExportResponseMembershipsItemBuilder::company_name)
    /// - [`role`](PostV1AccountExportResponseMembershipsItemBuilder::role)
    /// - [`since`](PostV1AccountExportResponseMembershipsItemBuilder::since)
    pub fn build(self) -> Result<PostV1AccountExportResponseMembershipsItem, BuildError> {
        Ok(PostV1AccountExportResponseMembershipsItem {
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
