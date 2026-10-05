pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAuditResponseRowsItem {
    #[serde(default)]
    pub id: i64,
    #[serde(rename = "actorType")]
    pub actor_type: ListAuditResponseRowsItemActorType,
    #[serde(rename = "actorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub entity: String,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ListAuditResponseRowsItem {
    pub fn builder() -> ListAuditResponseRowsItemBuilder {
        <ListAuditResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuditResponseRowsItemBuilder {
    id: Option<i64>,
    actor_type: Option<ListAuditResponseRowsItemActorType>,
    actor_id: Option<String>,
    action: Option<String>,
    entity: Option<String>,
    entity_id: Option<String>,
    diff: Option<serde_json::Value>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ListAuditResponseRowsItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn actor_type(mut self, value: ListAuditResponseRowsItemActorType) -> Self {
        self.actor_type = Some(value);
        self
    }

    pub fn actor_id(mut self, value: impl Into<String>) -> Self {
        self.actor_id = Some(value.into());
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

    pub fn diff(mut self, value: serde_json::Value) -> Self {
        self.diff = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAuditResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListAuditResponseRowsItemBuilder::id)
    /// - [`actor_type`](ListAuditResponseRowsItemBuilder::actor_type)
    /// - [`action`](ListAuditResponseRowsItemBuilder::action)
    /// - [`entity`](ListAuditResponseRowsItemBuilder::entity)
    /// - [`created_at`](ListAuditResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<ListAuditResponseRowsItem, BuildError> {
        Ok(ListAuditResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            actor_type: self
                .actor_type
                .ok_or_else(|| BuildError::missing_field("actor_type"))?,
            actor_id: self.actor_id,
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            entity: self
                .entity
                .ok_or_else(|| BuildError::missing_field("entity"))?,
            entity_id: self.entity_id,
            diff: self.diff,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
