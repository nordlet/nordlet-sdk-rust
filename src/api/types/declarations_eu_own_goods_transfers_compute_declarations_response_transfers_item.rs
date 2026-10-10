pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItem {
    #[serde(rename = "movementId")]
    #[serde(default)]
    pub movement_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(default)]
    pub cost: String,
    #[serde(rename = "fromCountryCode")]
    #[serde(default)]
    pub from_country_code: String,
    #[serde(rename = "toCountryCode")]
    #[serde(default)]
    pub to_country_code: String,
}

impl EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItem {
    pub fn builder() -> EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder {
        <EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder {
    movement_id: Option<String>,
    date: Option<NaiveDate>,
    item_id: Option<String>,
    item_name: Option<String>,
    quantity: Option<String>,
    cost: Option<String>,
    from_country_code: Option<String>,
    to_country_code: Option<String>,
}

impl EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder {
    pub fn movement_id(mut self, value: impl Into<String>) -> Self {
        self.movement_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.item_name = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn cost(mut self, value: impl Into<String>) -> Self {
        self.cost = Some(value.into());
        self
    }

    pub fn from_country_code(mut self, value: impl Into<String>) -> Self {
        self.from_country_code = Some(value.into());
        self
    }

    pub fn to_country_code(mut self, value: impl Into<String>) -> Self {
        self.to_country_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`movement_id`](EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder::movement_id)
    /// - [`date`](EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder::date)
    /// - [`item_id`](EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder::item_id)
    /// - [`item_name`](EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder::item_name)
    /// - [`quantity`](EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder::quantity)
    /// - [`cost`](EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder::cost)
    /// - [`from_country_code`](EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder::from_country_code)
    /// - [`to_country_code`](EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItemBuilder::to_country_code)
    pub fn build(
        self,
    ) -> Result<EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItem, BuildError> {
        Ok(
            EuOwnGoodsTransfersComputeDeclarationsResponseTransfersItem {
                movement_id: self
                    .movement_id
                    .ok_or_else(|| BuildError::missing_field("movement_id"))?,
                date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
                item_id: self
                    .item_id
                    .ok_or_else(|| BuildError::missing_field("item_id"))?,
                item_name: self
                    .item_name
                    .ok_or_else(|| BuildError::missing_field("item_name"))?,
                quantity: self
                    .quantity
                    .ok_or_else(|| BuildError::missing_field("quantity"))?,
                cost: self.cost.ok_or_else(|| BuildError::missing_field("cost"))?,
                from_country_code: self
                    .from_country_code
                    .ok_or_else(|| BuildError::missing_field("from_country_code"))?,
                to_country_code: self
                    .to_country_code
                    .ok_or_else(|| BuildError::missing_field("to_country_code"))?,
            },
        )
    }
}
