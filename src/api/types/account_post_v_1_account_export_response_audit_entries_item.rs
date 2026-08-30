pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountExportResponseAuditEntriesItem {
    #[serde(default)]
    pub id: i64,
    #[serde(rename = "companyId")]
    #[serde(default)]
    pub company_id: String,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub entity: String,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1AccountExportResponseAuditEntriesItem {
    pub fn builder() -> PostV1AccountExportResponseAuditEntriesItemBuilder {
        <PostV1AccountExportResponseAuditEntriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountExportResponseAuditEntriesItemBuilder {
    id: Option<i64>,
    company_id: Option<String>,
    action: Option<String>,
    entity: Option<String>,
    entity_id: Option<String>,
    created_at: Option<String>,
}

impl PostV1AccountExportResponseAuditEntriesItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn entity(mut self, value: impl Into<String>) -> Self {
        self.entity = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountExportResponseAuditEntriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountExportResponseAuditEntriesItemBuilder::id)
    /// - [`company_id`](PostV1AccountExportResponseAuditEntriesItemBuilder::company_id)
    /// - [`action`](PostV1AccountExportResponseAuditEntriesItemBuilder::action)
    /// - [`entity`](PostV1AccountExportResponseAuditEntriesItemBuilder::entity)
    /// - [`created_at`](PostV1AccountExportResponseAuditEntriesItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1AccountExportResponseAuditEntriesItem, BuildError> {
        Ok(PostV1AccountExportResponseAuditEntriesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            entity: self
                .entity
                .ok_or_else(|| BuildError::missing_field("entity"))?,
            entity_id: self.entity_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
