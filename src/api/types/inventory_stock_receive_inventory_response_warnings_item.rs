pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockReceiveInventoryResponseWarningsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub fix: String,
}

impl StockReceiveInventoryResponseWarningsItem {
    pub fn builder() -> StockReceiveInventoryResponseWarningsItemBuilder {
        <StockReceiveInventoryResponseWarningsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockReceiveInventoryResponseWarningsItemBuilder {
    code: Option<String>,
    message: Option<String>,
    fix: Option<String>,
}

impl StockReceiveInventoryResponseWarningsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn fix(mut self, value: impl Into<String>) -> Self {
        self.fix = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockReceiveInventoryResponseWarningsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](StockReceiveInventoryResponseWarningsItemBuilder::code)
    /// - [`message`](StockReceiveInventoryResponseWarningsItemBuilder::message)
    /// - [`fix`](StockReceiveInventoryResponseWarningsItemBuilder::fix)
    pub fn build(self) -> Result<StockReceiveInventoryResponseWarningsItem, BuildError> {
        Ok(StockReceiveInventoryResponseWarningsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            fix: self.fix.ok_or_else(|| BuildError::missing_field("fix"))?,
        })
    }
}
