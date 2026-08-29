pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLotsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "lotNumber")]
    #[serde(default)]
    pub lot_number: String,
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "onHand")]
    #[serde(default)]
    pub on_hand: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1InventoryLotsListResponseRowsItem {
    pub fn builder() -> PostV1InventoryLotsListResponseRowsItemBuilder {
        <PostV1InventoryLotsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLotsListResponseRowsItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    lot_number: Option<String>,
    expiry_date: Option<String>,
    notes: Option<String>,
    on_hand: Option<String>,
    created_at: Option<String>,
}

impl PostV1InventoryLotsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn expiry_date(mut self, value: impl Into<String>) -> Self {
        self.expiry_date = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn on_hand(mut self, value: impl Into<String>) -> Self {
        self.on_hand = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLotsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryLotsListResponseRowsItemBuilder::id)
    /// - [`item_id`](PostV1InventoryLotsListResponseRowsItemBuilder::item_id)
    /// - [`lot_number`](PostV1InventoryLotsListResponseRowsItemBuilder::lot_number)
    /// - [`on_hand`](PostV1InventoryLotsListResponseRowsItemBuilder::on_hand)
    /// - [`created_at`](PostV1InventoryLotsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1InventoryLotsListResponseRowsItem, BuildError> {
        Ok(PostV1InventoryLotsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            lot_number: self
                .lot_number
                .ok_or_else(|| BuildError::missing_field("lot_number"))?,
            expiry_date: self.expiry_date,
            notes: self.notes,
            on_hand: self
                .on_hand
                .ok_or_else(|| BuildError::missing_field("on_hand"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
