pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BomsListProductionResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "finishedItemId")]
    #[serde(default)]
    pub finished_item_id: String,
    #[serde(rename = "outputQuantity")]
    #[serde(default)]
    pub output_quantity: String,
    #[serde(rename = "routingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_id: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
}

impl BomsListProductionResponseRowsItem {
    pub fn builder() -> BomsListProductionResponseRowsItemBuilder {
        <BomsListProductionResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BomsListProductionResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    finished_item_id: Option<String>,
    output_quantity: Option<String>,
    routing_id: Option<String>,
    is_active: Option<bool>,
}

impl BomsListProductionResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn finished_item_id(mut self, value: impl Into<String>) -> Self {
        self.finished_item_id = Some(value.into());
        self
    }

    pub fn output_quantity(mut self, value: impl Into<String>) -> Self {
        self.output_quantity = Some(value.into());
        self
    }

    pub fn routing_id(mut self, value: impl Into<String>) -> Self {
        self.routing_id = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BomsListProductionResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BomsListProductionResponseRowsItemBuilder::id)
    /// - [`code`](BomsListProductionResponseRowsItemBuilder::code)
    /// - [`name`](BomsListProductionResponseRowsItemBuilder::name)
    /// - [`finished_item_id`](BomsListProductionResponseRowsItemBuilder::finished_item_id)
    /// - [`output_quantity`](BomsListProductionResponseRowsItemBuilder::output_quantity)
    /// - [`is_active`](BomsListProductionResponseRowsItemBuilder::is_active)
    pub fn build(self) -> Result<BomsListProductionResponseRowsItem, BuildError> {
        Ok(BomsListProductionResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            finished_item_id: self
                .finished_item_id
                .ok_or_else(|| BuildError::missing_field("finished_item_id"))?,
            output_quantity: self
                .output_quantity
                .ok_or_else(|| BuildError::missing_field("output_quantity"))?,
            routing_id: self.routing_id,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
        })
    }
}
