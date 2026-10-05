pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtAgingReportsResponseRowsItem {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(default)]
    pub current: String,
    #[serde(default)]
    pub d1to30: String,
    #[serde(default)]
    pub d31to60: String,
    #[serde(default)]
    pub d61to90: String,
    #[serde(default)]
    pub over90: String,
    #[serde(default)]
    pub total: String,
}

impl DebtAgingReportsResponseRowsItem {
    pub fn builder() -> DebtAgingReportsResponseRowsItemBuilder {
        <DebtAgingReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtAgingReportsResponseRowsItemBuilder {
    partner_id: Option<String>,
    partner_name: Option<String>,
    current: Option<String>,
    d1to30: Option<String>,
    d31to60: Option<String>,
    d61to90: Option<String>,
    over90: Option<String>,
    total: Option<String>,
}

impl DebtAgingReportsResponseRowsItemBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn current(mut self, value: impl Into<String>) -> Self {
        self.current = Some(value.into());
        self
    }

    pub fn d1to30(mut self, value: impl Into<String>) -> Self {
        self.d1to30 = Some(value.into());
        self
    }

    pub fn d31to60(mut self, value: impl Into<String>) -> Self {
        self.d31to60 = Some(value.into());
        self
    }

    pub fn d61to90(mut self, value: impl Into<String>) -> Self {
        self.d61to90 = Some(value.into());
        self
    }

    pub fn over90(mut self, value: impl Into<String>) -> Self {
        self.over90 = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DebtAgingReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](DebtAgingReportsResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](DebtAgingReportsResponseRowsItemBuilder::partner_name)
    /// - [`current`](DebtAgingReportsResponseRowsItemBuilder::current)
    /// - [`d1to30`](DebtAgingReportsResponseRowsItemBuilder::d1to30)
    /// - [`d31to60`](DebtAgingReportsResponseRowsItemBuilder::d31to60)
    /// - [`d61to90`](DebtAgingReportsResponseRowsItemBuilder::d61to90)
    /// - [`over90`](DebtAgingReportsResponseRowsItemBuilder::over90)
    /// - [`total`](DebtAgingReportsResponseRowsItemBuilder::total)
    pub fn build(self) -> Result<DebtAgingReportsResponseRowsItem, BuildError> {
        Ok(DebtAgingReportsResponseRowsItem {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            current: self
                .current
                .ok_or_else(|| BuildError::missing_field("current"))?,
            d1to30: self
                .d1to30
                .ok_or_else(|| BuildError::missing_field("d1to30"))?,
            d31to60: self
                .d31to60
                .ok_or_else(|| BuildError::missing_field("d31to60"))?,
            d61to90: self
                .d61to90
                .ok_or_else(|| BuildError::missing_field("d61to90"))?,
            over90: self
                .over90
                .ok_or_else(|| BuildError::missing_field("over90"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
