pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1AuditListResponseRowsItem {
    #[serde(default)]
    pub id: i64,
    #[serde(rename = "actorType")]
    pub actor_type: PostV1AuditListResponseRowsItemActorType,
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
    pub created_at: String,
}

impl PostV1AuditListResponseRowsItem {
    pub fn builder() -> PostV1AuditListResponseRowsItemBuilder {
        <PostV1AuditListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AuditListResponseRowsItemBuilder {
    id: Option<i64>,
    actor_type: Option<PostV1AuditListResponseRowsItemActorType>,
    actor_id: Option<String>,
    action: Option<String>,
    entity: Option<String>,
    entity_id: Option<String>,
    diff: Option<serde_json::Value>,
    created_at: Option<String>,
}

impl PostV1AuditListResponseRowsItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn actor_type(mut self, value: PostV1AuditListResponseRowsItemActorType) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AuditListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AuditListResponseRowsItemBuilder::id)
    /// - [`actor_type`](PostV1AuditListResponseRowsItemBuilder::actor_type)
    /// - [`action`](PostV1AuditListResponseRowsItemBuilder::action)
    /// - [`entity`](PostV1AuditListResponseRowsItemBuilder::entity)
    /// - [`created_at`](PostV1AuditListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1AuditListResponseRowsItem, BuildError> {
        Ok(PostV1AuditListResponseRowsItem {
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
