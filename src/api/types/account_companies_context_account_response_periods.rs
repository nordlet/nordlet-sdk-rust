pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesContextAccountResponsePeriods {
    #[serde(default)]
    pub open: Vec<CompaniesContextAccountResponsePeriodsOpenItem>,
    #[serde(default)]
    pub locked: Vec<CompaniesContextAccountResponsePeriodsLockedItem>,
}

impl CompaniesContextAccountResponsePeriods {
    pub fn builder() -> CompaniesContextAccountResponsePeriodsBuilder {
        <CompaniesContextAccountResponsePeriodsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesContextAccountResponsePeriodsBuilder {
    open: Option<Vec<CompaniesContextAccountResponsePeriodsOpenItem>>,
    locked: Option<Vec<CompaniesContextAccountResponsePeriodsLockedItem>>,
}

impl CompaniesContextAccountResponsePeriodsBuilder {
    pub fn open(mut self, value: Vec<CompaniesContextAccountResponsePeriodsOpenItem>) -> Self {
        self.open = Some(value);
        self
    }

    pub fn locked(mut self, value: Vec<CompaniesContextAccountResponsePeriodsLockedItem>) -> Self {
        self.locked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesContextAccountResponsePeriods`].
    /// This method will fail if any of the following fields are not set:
    /// - [`open`](CompaniesContextAccountResponsePeriodsBuilder::open)
    /// - [`locked`](CompaniesContextAccountResponsePeriodsBuilder::locked)
    pub fn build(self) -> Result<CompaniesContextAccountResponsePeriods, BuildError> {
        Ok(CompaniesContextAccountResponsePeriods {
            open: self.open.ok_or_else(|| BuildError::missing_field("open"))?,
            locked: self
                .locked
                .ok_or_else(|| BuildError::missing_field("locked"))?,
        })
    }
}
