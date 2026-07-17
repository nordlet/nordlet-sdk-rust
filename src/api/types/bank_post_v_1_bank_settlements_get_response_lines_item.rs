pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsGetResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "externalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub gross: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub net: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "chargeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "matchedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_invoice_id: Option<String>,
    #[serde(rename = "matchStatus")]
    pub match_status: PostV1BankSettlementsGetResponseLinesItemMatchStatus,
}

impl PostV1BankSettlementsGetResponseLinesItem {
    pub fn builder() -> PostV1BankSettlementsGetResponseLinesItemBuilder {
        <PostV1BankSettlementsGetResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsGetResponseLinesItemBuilder {
    id: Option<String>,
    external_id: Option<String>,
    category: Option<String>,
    date: Option<String>,
    gross: Option<String>,
    fee: Option<String>,
    net: Option<String>,
    description: Option<String>,
    source_id: Option<String>,
    charge_id: Option<String>,
    reference: Option<String>,
    matched_invoice_id: Option<String>,
    match_status: Option<PostV1BankSettlementsGetResponseLinesItemMatchStatus>,
}

impl PostV1BankSettlementsGetResponseLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    pub fn fee(mut self, value: impl Into<String>) -> Self {
        self.fee = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn charge_id(mut self, value: impl Into<String>) -> Self {
        self.charge_id = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn matched_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.matched_invoice_id = Some(value.into());
        self
    }

    pub fn match_status(
        mut self,
        value: PostV1BankSettlementsGetResponseLinesItemMatchStatus,
    ) -> Self {
        self.match_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsGetResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankSettlementsGetResponseLinesItemBuilder::id)
    /// - [`external_id`](PostV1BankSettlementsGetResponseLinesItemBuilder::external_id)
    /// - [`category`](PostV1BankSettlementsGetResponseLinesItemBuilder::category)
    /// - [`date`](PostV1BankSettlementsGetResponseLinesItemBuilder::date)
    /// - [`gross`](PostV1BankSettlementsGetResponseLinesItemBuilder::gross)
    /// - [`fee`](PostV1BankSettlementsGetResponseLinesItemBuilder::fee)
    /// - [`net`](PostV1BankSettlementsGetResponseLinesItemBuilder::net)
    /// - [`match_status`](PostV1BankSettlementsGetResponseLinesItemBuilder::match_status)
    pub fn build(self) -> Result<PostV1BankSettlementsGetResponseLinesItem, BuildError> {
        Ok(PostV1BankSettlementsGetResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
            fee: self.fee.ok_or_else(|| BuildError::missing_field("fee"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            description: self.description,
            source_id: self.source_id,
            charge_id: self.charge_id,
            reference: self.reference,
            matched_invoice_id: self.matched_invoice_id,
            match_status: self
                .match_status
                .ok_or_else(|| BuildError::missing_field("match_status"))?,
        })
    }
}
