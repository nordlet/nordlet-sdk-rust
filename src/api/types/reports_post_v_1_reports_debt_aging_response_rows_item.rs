pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsDebtAgingResponseRowsItem {
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

impl PostV1ReportsDebtAgingResponseRowsItem {
    pub fn builder() -> PostV1ReportsDebtAgingResponseRowsItemBuilder {
        <PostV1ReportsDebtAgingResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsDebtAgingResponseRowsItemBuilder {
    partner_id: Option<String>,
    partner_name: Option<String>,
    current: Option<String>,
    d1to30: Option<String>,
    d31to60: Option<String>,
    d61to90: Option<String>,
    over90: Option<String>,
    total: Option<String>,
}

impl PostV1ReportsDebtAgingResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsDebtAgingResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1ReportsDebtAgingResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](PostV1ReportsDebtAgingResponseRowsItemBuilder::partner_name)
    /// - [`current`](PostV1ReportsDebtAgingResponseRowsItemBuilder::current)
    /// - [`d1to30`](PostV1ReportsDebtAgingResponseRowsItemBuilder::d1to30)
    /// - [`d31to60`](PostV1ReportsDebtAgingResponseRowsItemBuilder::d31to60)
    /// - [`d61to90`](PostV1ReportsDebtAgingResponseRowsItemBuilder::d61to90)
    /// - [`over90`](PostV1ReportsDebtAgingResponseRowsItemBuilder::over90)
    /// - [`total`](PostV1ReportsDebtAgingResponseRowsItemBuilder::total)
    pub fn build(self) -> Result<PostV1ReportsDebtAgingResponseRowsItem, BuildError> {
        Ok(PostV1ReportsDebtAgingResponseRowsItem {
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
