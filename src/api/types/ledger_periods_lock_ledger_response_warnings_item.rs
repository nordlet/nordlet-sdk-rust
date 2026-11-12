pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeriodsLockLedgerResponseWarningsItem {
    pub code: PeriodsLockLedgerResponseWarningsItemCode,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub ids: Vec<String>,
}

impl PeriodsLockLedgerResponseWarningsItem {
    pub fn builder() -> PeriodsLockLedgerResponseWarningsItemBuilder {
        <PeriodsLockLedgerResponseWarningsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodsLockLedgerResponseWarningsItemBuilder {
    code: Option<PeriodsLockLedgerResponseWarningsItemCode>,
    count: Option<i64>,
    message: Option<String>,
    ids: Option<Vec<String>>,
}

impl PeriodsLockLedgerResponseWarningsItemBuilder {
    pub fn code(mut self, value: PeriodsLockLedgerResponseWarningsItemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn ids(mut self, value: Vec<String>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeriodsLockLedgerResponseWarningsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PeriodsLockLedgerResponseWarningsItemBuilder::code)
    /// - [`count`](PeriodsLockLedgerResponseWarningsItemBuilder::count)
    /// - [`message`](PeriodsLockLedgerResponseWarningsItemBuilder::message)
    /// - [`ids`](PeriodsLockLedgerResponseWarningsItemBuilder::ids)
    pub fn build(self) -> Result<PeriodsLockLedgerResponseWarningsItem, BuildError> {
        Ok(PeriodsLockLedgerResponseWarningsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
        })
    }
}
