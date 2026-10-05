pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunsCreatePayrollResponseComponentTotalsItem {
    #[serde(default)]
    pub code: String,
    pub kind: RunsCreatePayrollResponseComponentTotalsItemKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
}

impl RunsCreatePayrollResponseComponentTotalsItem {
    pub fn builder() -> RunsCreatePayrollResponseComponentTotalsItemBuilder {
        <RunsCreatePayrollResponseComponentTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCreatePayrollResponseComponentTotalsItemBuilder {
    code: Option<String>,
    kind: Option<RunsCreatePayrollResponseComponentTotalsItemKind>,
    amount: Option<String>,
    rate: Option<String>,
    base: Option<String>,
}

impl RunsCreatePayrollResponseComponentTotalsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn kind(mut self, value: RunsCreatePayrollResponseComponentTotalsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsCreatePayrollResponseComponentTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](RunsCreatePayrollResponseComponentTotalsItemBuilder::code)
    /// - [`kind`](RunsCreatePayrollResponseComponentTotalsItemBuilder::kind)
    /// - [`amount`](RunsCreatePayrollResponseComponentTotalsItemBuilder::amount)
    pub fn build(self) -> Result<RunsCreatePayrollResponseComponentTotalsItem, BuildError> {
        Ok(RunsCreatePayrollResponseComponentTotalsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            rate: self.rate,
            base: self.base,
        })
    }
}
