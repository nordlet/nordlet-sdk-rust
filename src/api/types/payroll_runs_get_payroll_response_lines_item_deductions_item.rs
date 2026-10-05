pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsGetPayrollResponseLinesItemDeductionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
}

impl RunsGetPayrollResponseLinesItemDeductionsItem {
    pub fn builder() -> RunsGetPayrollResponseLinesItemDeductionsItemBuilder {
        <RunsGetPayrollResponseLinesItemDeductionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsGetPayrollResponseLinesItemDeductionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
}

impl RunsGetPayrollResponseLinesItemDeductionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsGetPayrollResponseLinesItemDeductionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RunsGetPayrollResponseLinesItemDeductionsItemBuilder::name)
    /// - [`amount`](RunsGetPayrollResponseLinesItemDeductionsItemBuilder::amount)
    pub fn build(self) -> Result<RunsGetPayrollResponseLinesItemDeductionsItem, BuildError> {
        Ok(RunsGetPayrollResponseLinesItemDeductionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
