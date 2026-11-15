pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueueListReviewResponseItemsItem {
    pub kind: QueueListReviewResponseItemsItemKind,
    /// The id of the draft, bank transaction or partner, or the key of the filing
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub fix: String,
    /// The warning code of a draft_warning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl QueueListReviewResponseItemsItem {
    pub fn builder() -> QueueListReviewResponseItemsItemBuilder {
        <QueueListReviewResponseItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueueListReviewResponseItemsItemBuilder {
    kind: Option<QueueListReviewResponseItemsItemKind>,
    id: Option<String>,
    message: Option<String>,
    fix: Option<String>,
    code: Option<String>,
}

impl QueueListReviewResponseItemsItemBuilder {
    pub fn kind(mut self, value: QueueListReviewResponseItemsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn fix(mut self, value: impl Into<String>) -> Self {
        self.fix = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueueListReviewResponseItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueueListReviewResponseItemsItemBuilder::kind)
    /// - [`id`](QueueListReviewResponseItemsItemBuilder::id)
    /// - [`message`](QueueListReviewResponseItemsItemBuilder::message)
    /// - [`fix`](QueueListReviewResponseItemsItemBuilder::fix)
    pub fn build(self) -> Result<QueueListReviewResponseItemsItem, BuildError> {
        Ok(QueueListReviewResponseItemsItem {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            fix: self.fix.ok_or_else(|| BuildError::missing_field("fix"))?,
            code: self.code,
        })
    }
}
