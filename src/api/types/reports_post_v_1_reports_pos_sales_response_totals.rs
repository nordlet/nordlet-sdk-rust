pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsPosSalesResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub gross: String,
    #[serde(default)]
    pub cash: String,
    #[serde(default)]
    pub card: String,
    #[serde(default)]
    pub cogs: String,
}

impl PostV1ReportsPosSalesResponseTotals {
    pub fn builder() -> PostV1ReportsPosSalesResponseTotalsBuilder {
        <PostV1ReportsPosSalesResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsPosSalesResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
    cash: Option<String>,
    card: Option<String>,
    cogs: Option<String>,
}

impl PostV1ReportsPosSalesResponseTotalsBuilder {
    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    pub fn cash(mut self, value: impl Into<String>) -> Self {
        self.cash = Some(value.into());
        self
    }

    pub fn card(mut self, value: impl Into<String>) -> Self {
        self.card = Some(value.into());
        self
    }

    pub fn cogs(mut self, value: impl Into<String>) -> Self {
        self.cogs = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsPosSalesResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](PostV1ReportsPosSalesResponseTotalsBuilder::net)
    /// - [`vat`](PostV1ReportsPosSalesResponseTotalsBuilder::vat)
    /// - [`gross`](PostV1ReportsPosSalesResponseTotalsBuilder::gross)
    /// - [`cash`](PostV1ReportsPosSalesResponseTotalsBuilder::cash)
    /// - [`card`](PostV1ReportsPosSalesResponseTotalsBuilder::card)
    /// - [`cogs`](PostV1ReportsPosSalesResponseTotalsBuilder::cogs)
    pub fn build(self) -> Result<PostV1ReportsPosSalesResponseTotals, BuildError> {
        Ok(PostV1ReportsPosSalesResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
            cash: self.cash.ok_or_else(|| BuildError::missing_field("cash"))?,
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            cogs: self.cogs.ok_or_else(|| BuildError::missing_field("cogs"))?,
        })
    }
}
