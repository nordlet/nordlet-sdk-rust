pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsPosSalesResponseRowsItem {
    #[serde(rename = "reportId")]
    #[serde(default)]
    pub report_id: String,
    #[serde(rename = "reportNumber")]
    #[serde(default)]
    pub report_number: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub gross: String,
    #[serde(default)]
    pub cash: String,
    #[serde(default)]
    pub card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cogs: Option<String>,
}

impl PostV1ReportsPosSalesResponseRowsItem {
    pub fn builder() -> PostV1ReportsPosSalesResponseRowsItemBuilder {
        <PostV1ReportsPosSalesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsPosSalesResponseRowsItemBuilder {
    report_id: Option<String>,
    report_number: Option<String>,
    date: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
    cash: Option<String>,
    card: Option<String>,
    cogs: Option<String>,
}

impl PostV1ReportsPosSalesResponseRowsItemBuilder {
    pub fn report_id(mut self, value: impl Into<String>) -> Self {
        self.report_id = Some(value.into());
        self
    }

    pub fn report_number(mut self, value: impl Into<String>) -> Self {
        self.report_number = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    pub fn cash(mut self, value: impl Into<String>) -> Self {
        self.cash = Some(value.into());
        self
    }

    pub fn card(mut self, value: impl Into<String>) -> Self {
        self.card = Some(value.into());
        self
    }

    pub fn cogs(mut self, value: impl Into<String>) -> Self {
        self.cogs = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsPosSalesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`report_id`](PostV1ReportsPosSalesResponseRowsItemBuilder::report_id)
    /// - [`report_number`](PostV1ReportsPosSalesResponseRowsItemBuilder::report_number)
    /// - [`date`](PostV1ReportsPosSalesResponseRowsItemBuilder::date)
    /// - [`net`](PostV1ReportsPosSalesResponseRowsItemBuilder::net)
    /// - [`vat`](PostV1ReportsPosSalesResponseRowsItemBuilder::vat)
    /// - [`gross`](PostV1ReportsPosSalesResponseRowsItemBuilder::gross)
    /// - [`cash`](PostV1ReportsPosSalesResponseRowsItemBuilder::cash)
    /// - [`card`](PostV1ReportsPosSalesResponseRowsItemBuilder::card)
    pub fn build(self) -> Result<PostV1ReportsPosSalesResponseRowsItem, BuildError> {
        Ok(PostV1ReportsPosSalesResponseRowsItem {
            report_id: self
                .report_id
                .ok_or_else(|| BuildError::missing_field("report_id"))?,
            report_number: self
                .report_number
                .ok_or_else(|| BuildError::missing_field("report_number"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
            cash: self.cash.ok_or_else(|| BuildError::missing_field("cash"))?,
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            cogs: self.cogs,
        })
    }
}
