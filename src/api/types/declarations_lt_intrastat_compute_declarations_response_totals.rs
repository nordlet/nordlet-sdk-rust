pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIntrastatComputeDeclarationsResponseTotals {
    #[serde(rename = "invoicedValue")]
    #[serde(default)]
    pub invoiced_value: String,
    #[serde(rename = "statisticalValue")]
    #[serde(default)]
    pub statistical_value: String,
    #[serde(rename = "netMassKg")]
    #[serde(default)]
    pub net_mass_kg: String,
    #[serde(default)]
    pub lines: i64,
}

impl LtIntrastatComputeDeclarationsResponseTotals {
    pub fn builder() -> LtIntrastatComputeDeclarationsResponseTotalsBuilder {
        <LtIntrastatComputeDeclarationsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIntrastatComputeDeclarationsResponseTotalsBuilder {
    invoiced_value: Option<String>,
    statistical_value: Option<String>,
    net_mass_kg: Option<String>,
    lines: Option<i64>,
}

impl LtIntrastatComputeDeclarationsResponseTotalsBuilder {
    pub fn invoiced_value(mut self, value: impl Into<String>) -> Self {
        self.invoiced_value = Some(value.into());
        self
    }

    pub fn statistical_value(mut self, value: impl Into<String>) -> Self {
        self.statistical_value = Some(value.into());
        self
    }

    pub fn net_mass_kg(mut self, value: impl Into<String>) -> Self {
        self.net_mass_kg = Some(value.into());
        self
    }

    pub fn lines(mut self, value: i64) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIntrastatComputeDeclarationsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoiced_value`](LtIntrastatComputeDeclarationsResponseTotalsBuilder::invoiced_value)
    /// - [`statistical_value`](LtIntrastatComputeDeclarationsResponseTotalsBuilder::statistical_value)
    /// - [`net_mass_kg`](LtIntrastatComputeDeclarationsResponseTotalsBuilder::net_mass_kg)
    /// - [`lines`](LtIntrastatComputeDeclarationsResponseTotalsBuilder::lines)
    pub fn build(self) -> Result<LtIntrastatComputeDeclarationsResponseTotals, BuildError> {
        Ok(LtIntrastatComputeDeclarationsResponseTotals {
            invoiced_value: self
                .invoiced_value
                .ok_or_else(|| BuildError::missing_field("invoiced_value"))?,
            statistical_value: self
                .statistical_value
                .ok_or_else(|| BuildError::missing_field("statistical_value"))?,
            net_mass_kg: self
                .net_mass_kg
                .ok_or_else(|| BuildError::missing_field("net_mass_kg"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
