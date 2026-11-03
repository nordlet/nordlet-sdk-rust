pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatClassifiersListReferenceResponseRowsItemRateFromLine {
    #[serde(default)]
    pub sale: bool,
    #[serde(default)]
    pub purchase: bool,
}

impl VatClassifiersListReferenceResponseRowsItemRateFromLine {
    pub fn builder() -> VatClassifiersListReferenceResponseRowsItemRateFromLineBuilder {
        <VatClassifiersListReferenceResponseRowsItemRateFromLineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatClassifiersListReferenceResponseRowsItemRateFromLineBuilder {
    sale: Option<bool>,
    purchase: Option<bool>,
}

impl VatClassifiersListReferenceResponseRowsItemRateFromLineBuilder {
    pub fn sale(mut self, value: bool) -> Self {
        self.sale = Some(value);
        self
    }

    pub fn purchase(mut self, value: bool) -> Self {
        self.purchase = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatClassifiersListReferenceResponseRowsItemRateFromLine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sale`](VatClassifiersListReferenceResponseRowsItemRateFromLineBuilder::sale)
    /// - [`purchase`](VatClassifiersListReferenceResponseRowsItemRateFromLineBuilder::purchase)
    pub fn build(
        self,
    ) -> Result<VatClassifiersListReferenceResponseRowsItemRateFromLine, BuildError> {
        Ok(VatClassifiersListReferenceResponseRowsItemRateFromLine {
            sale: self.sale.ok_or_else(|| BuildError::missing_field("sale"))?,
            purchase: self
                .purchase
                .ok_or_else(|| BuildError::missing_field("purchase"))?,
        })
    }
}
