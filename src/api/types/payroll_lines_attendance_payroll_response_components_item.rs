pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LinesAttendancePayrollResponseComponentsItem {
    #[serde(default)]
    pub code: String,
    pub kind: LinesAttendancePayrollResponseComponentsItemKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
}

impl LinesAttendancePayrollResponseComponentsItem {
    pub fn builder() -> LinesAttendancePayrollResponseComponentsItemBuilder {
        <LinesAttendancePayrollResponseComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinesAttendancePayrollResponseComponentsItemBuilder {
    code: Option<String>,
    kind: Option<LinesAttendancePayrollResponseComponentsItemKind>,
    amount: Option<String>,
    rate: Option<String>,
    base: Option<String>,
}

impl LinesAttendancePayrollResponseComponentsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn kind(mut self, value: LinesAttendancePayrollResponseComponentsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`LinesAttendancePayrollResponseComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](LinesAttendancePayrollResponseComponentsItemBuilder::code)
    /// - [`kind`](LinesAttendancePayrollResponseComponentsItemBuilder::kind)
    /// - [`amount`](LinesAttendancePayrollResponseComponentsItemBuilder::amount)
    pub fn build(self) -> Result<LinesAttendancePayrollResponseComponentsItem, BuildError> {
        Ok(LinesAttendancePayrollResponseComponentsItem {
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
