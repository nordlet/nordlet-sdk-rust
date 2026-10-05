pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CalcPayrollResponseComponentsItem {
    #[serde(default)]
    pub code: String,
    pub kind: CalcPayrollResponseComponentsItemKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
}

impl CalcPayrollResponseComponentsItem {
    pub fn builder() -> CalcPayrollResponseComponentsItemBuilder {
        <CalcPayrollResponseComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalcPayrollResponseComponentsItemBuilder {
    code: Option<String>,
    kind: Option<CalcPayrollResponseComponentsItemKind>,
    amount: Option<String>,
    rate: Option<String>,
    base: Option<String>,
}

impl CalcPayrollResponseComponentsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CalcPayrollResponseComponentsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`CalcPayrollResponseComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CalcPayrollResponseComponentsItemBuilder::code)
    /// - [`kind`](CalcPayrollResponseComponentsItemBuilder::kind)
    /// - [`amount`](CalcPayrollResponseComponentsItemBuilder::amount)
    pub fn build(self) -> Result<CalcPayrollResponseComponentsItem, BuildError> {
        Ok(CalcPayrollResponseComponentsItem {
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
