pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIvazAmendDeclarationsRequestCorrectionsItemLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub quantity: String,
}

impl LtIvazAmendDeclarationsRequestCorrectionsItemLinesItem {
    pub fn builder() -> LtIvazAmendDeclarationsRequestCorrectionsItemLinesItemBuilder {
        <LtIvazAmendDeclarationsRequestCorrectionsItemLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIvazAmendDeclarationsRequestCorrectionsItemLinesItemBuilder {
    id: Option<String>,
    quantity: Option<String>,
}

impl LtIvazAmendDeclarationsRequestCorrectionsItemLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtIvazAmendDeclarationsRequestCorrectionsItemLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LtIvazAmendDeclarationsRequestCorrectionsItemLinesItemBuilder::id)
    /// - [`quantity`](LtIvazAmendDeclarationsRequestCorrectionsItemLinesItemBuilder::quantity)
    pub fn build(
        self,
    ) -> Result<LtIvazAmendDeclarationsRequestCorrectionsItemLinesItem, BuildError> {
        Ok(LtIvazAmendDeclarationsRequestCorrectionsItemLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
