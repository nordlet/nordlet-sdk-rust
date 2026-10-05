pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliveriesListWebhooksRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<DeliveriesListWebhooksRequestSortItemDir>,
}

impl DeliveriesListWebhooksRequestSortItem {
    pub fn builder() -> DeliveriesListWebhooksRequestSortItemBuilder {
        <DeliveriesListWebhooksRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesListWebhooksRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<DeliveriesListWebhooksRequestSortItemDir>,
}

impl DeliveriesListWebhooksRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: DeliveriesListWebhooksRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesListWebhooksRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DeliveriesListWebhooksRequestSortItemBuilder::field)
    pub fn build(self) -> Result<DeliveriesListWebhooksRequestSortItem, BuildError> {
        Ok(DeliveriesListWebhooksRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
