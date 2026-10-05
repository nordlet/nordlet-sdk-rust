pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinesAttendancePayrollResponseAdditionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub taxable: bool,
}

impl LinesAttendancePayrollResponseAdditionsItem {
    pub fn builder() -> LinesAttendancePayrollResponseAdditionsItemBuilder {
        <LinesAttendancePayrollResponseAdditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinesAttendancePayrollResponseAdditionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
    taxable: Option<bool>,
}

impl LinesAttendancePayrollResponseAdditionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn taxable(mut self, value: bool) -> Self {
        self.taxable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinesAttendancePayrollResponseAdditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](LinesAttendancePayrollResponseAdditionsItemBuilder::name)
    /// - [`amount`](LinesAttendancePayrollResponseAdditionsItemBuilder::amount)
    /// - [`taxable`](LinesAttendancePayrollResponseAdditionsItemBuilder::taxable)
    pub fn build(self) -> Result<LinesAttendancePayrollResponseAdditionsItem, BuildError> {
        Ok(LinesAttendancePayrollResponseAdditionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            taxable: self
                .taxable
                .ok_or_else(|| BuildError::missing_field("taxable"))?,
        })
    }
}
