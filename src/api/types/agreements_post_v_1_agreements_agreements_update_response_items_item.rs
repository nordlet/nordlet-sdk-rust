pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsUpdateResponseItemsItem {
    #[serde(default)]
    pub id: String,
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
    #[serde(rename = "vatRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percent: Option<String>,
}

impl PostV1AgreementsAgreementsUpdateResponseItemsItem {
    pub fn builder() -> PostV1AgreementsAgreementsUpdateResponseItemsItemBuilder {
        <PostV1AgreementsAgreementsUpdateResponseItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsUpdateResponseItemsItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    unit_price: Option<String>,
    vat_rate_percent: Option<String>,
}

impl PostV1AgreementsAgreementsUpdateResponseItemsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsUpdateResponseItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsAgreementsUpdateResponseItemsItemBuilder::id)
    /// - [`description`](PostV1AgreementsAgreementsUpdateResponseItemsItemBuilder::description)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsUpdateResponseItemsItem, BuildError> {
        Ok(PostV1AgreementsAgreementsUpdateResponseItemsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self.item_id,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            quantity: self.quantity,
            unit_price: self.unit_price,
            vat_rate_percent: self.vat_rate_percent,
        })
    }
}
