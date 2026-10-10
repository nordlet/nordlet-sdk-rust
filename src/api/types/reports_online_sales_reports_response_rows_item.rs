pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OnlineSalesReportsResponseRowsItem {
    #[serde(default)]
    pub channel: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub orders: i64,
    #[serde(default)]
    pub fulfilled: i64,
    #[serde(default)]
    pub cancelled: i64,
    #[serde(default)]
    pub open: i64,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub gross: String,
}

impl OnlineSalesReportsResponseRowsItem {
    pub fn builder() -> OnlineSalesReportsResponseRowsItemBuilder {
        <OnlineSalesReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OnlineSalesReportsResponseRowsItemBuilder {
    channel: Option<String>,
    currency: Option<String>,
    orders: Option<i64>,
    fulfilled: Option<i64>,
    cancelled: Option<i64>,
    open: Option<i64>,
    net: Option<String>,
    gross: Option<String>,
}

impl OnlineSalesReportsResponseRowsItemBuilder {
    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn orders(mut self, value: i64) -> Self {
        self.orders = Some(value);
        self
    }

    pub fn fulfilled(mut self, value: i64) -> Self {
        self.fulfilled = Some(value);
        self
    }

    pub fn cancelled(mut self, value: i64) -> Self {
        self.cancelled = Some(value);
        self
    }

    pub fn open(mut self, value: i64) -> Self {
        self.open = Some(value);
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OnlineSalesReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel`](OnlineSalesReportsResponseRowsItemBuilder::channel)
    /// - [`currency`](OnlineSalesReportsResponseRowsItemBuilder::currency)
    /// - [`orders`](OnlineSalesReportsResponseRowsItemBuilder::orders)
    /// - [`fulfilled`](OnlineSalesReportsResponseRowsItemBuilder::fulfilled)
    /// - [`cancelled`](OnlineSalesReportsResponseRowsItemBuilder::cancelled)
    /// - [`open`](OnlineSalesReportsResponseRowsItemBuilder::open)
    /// - [`net`](OnlineSalesReportsResponseRowsItemBuilder::net)
    /// - [`gross`](OnlineSalesReportsResponseRowsItemBuilder::gross)
    pub fn build(self) -> Result<OnlineSalesReportsResponseRowsItem, BuildError> {
        Ok(OnlineSalesReportsResponseRowsItem {
            channel: self
                .channel
                .ok_or_else(|| BuildError::missing_field("channel"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            orders: self
                .orders
                .ok_or_else(|| BuildError::missing_field("orders"))?,
            fulfilled: self
                .fulfilled
                .ok_or_else(|| BuildError::missing_field("fulfilled"))?,
            cancelled: self
                .cancelled
                .ok_or_else(|| BuildError::missing_field("cancelled"))?,
            open: self.open.ok_or_else(|| BuildError::missing_field("open"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
        })
    }
}
