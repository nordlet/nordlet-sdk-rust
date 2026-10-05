pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinesAttendancePayrollResponseDeductionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
}

impl LinesAttendancePayrollResponseDeductionsItem {
    pub fn builder() -> LinesAttendancePayrollResponseDeductionsItemBuilder {
        <LinesAttendancePayrollResponseDeductionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinesAttendancePayrollResponseDeductionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
}

impl LinesAttendancePayrollResponseDeductionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinesAttendancePayrollResponseDeductionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](LinesAttendancePayrollResponseDeductionsItemBuilder::name)
    /// - [`amount`](LinesAttendancePayrollResponseDeductionsItemBuilder::amount)
    pub fn build(self) -> Result<LinesAttendancePayrollResponseDeductionsItem, BuildError> {
        Ok(LinesAttendancePayrollResponseDeductionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
