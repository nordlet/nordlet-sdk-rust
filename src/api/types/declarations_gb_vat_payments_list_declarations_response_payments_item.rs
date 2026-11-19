pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatPaymentsListDeclarationsResponsePaymentsItem {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<String>,
}

impl GbVatPaymentsListDeclarationsResponsePaymentsItem {
    pub fn builder() -> GbVatPaymentsListDeclarationsResponsePaymentsItemBuilder {
        <GbVatPaymentsListDeclarationsResponsePaymentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatPaymentsListDeclarationsResponsePaymentsItemBuilder {
    amount: Option<f64>,
    received: Option<String>,
}

impl GbVatPaymentsListDeclarationsResponsePaymentsItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn received(mut self, value: impl Into<String>) -> Self {
        self.received = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbVatPaymentsListDeclarationsResponsePaymentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](GbVatPaymentsListDeclarationsResponsePaymentsItemBuilder::amount)
    pub fn build(self) -> Result<GbVatPaymentsListDeclarationsResponsePaymentsItem, BuildError> {
        Ok(GbVatPaymentsListDeclarationsResponsePaymentsItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            received: self.received,
        })
    }
}
