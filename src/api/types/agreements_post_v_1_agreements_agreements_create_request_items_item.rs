pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsCreateRequestItemsItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
}

impl PostV1AgreementsAgreementsCreateRequestItemsItem {
    pub fn builder() -> PostV1AgreementsAgreementsCreateRequestItemsItemBuilder {
        <PostV1AgreementsAgreementsCreateRequestItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsCreateRequestItemsItemBuilder {
    item_id: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    unit_price: Option<String>,
}

impl PostV1AgreementsAgreementsCreateRequestItemsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_price(mut self, value: impl Into<String>) -> Self {
        self.unit_price = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsCreateRequestItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](PostV1AgreementsAgreementsCreateRequestItemsItemBuilder::description)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsCreateRequestItemsItem, BuildError> {
        Ok(PostV1AgreementsAgreementsCreateRequestItemsItem {
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            quantity: self.quantity,
            unit_price: self.unit_price,
        })
    }
}
