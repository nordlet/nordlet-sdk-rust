pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesImportsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "situationOn")]
    #[serde(default)]
    pub situation_on: String,
    pub status: PostV1ReferenceEuVatRatesImportsListResponseRowsItemStatus,
    pub trigger: PostV1ReferenceEuVatRatesImportsListResponseRowsItemTrigger,
    #[serde(rename = "ratesFetched")]
    #[serde(default)]
    pub rates_fetched: i64,
    #[serde(rename = "ratesInserted")]
    #[serde(default)]
    pub rates_inserted: i64,
    #[serde(rename = "ratesClosed")]
    #[serde(default)]
    pub rates_closed: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    pub started_at: String,
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

impl PostV1ReferenceEuVatRatesImportsListResponseRowsItem {
    pub fn builder() -> PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder {
        <PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder {
    id: Option<String>,
    situation_on: Option<String>,
    status: Option<PostV1ReferenceEuVatRatesImportsListResponseRowsItemStatus>,
    trigger: Option<PostV1ReferenceEuVatRatesImportsListResponseRowsItemTrigger>,
    rates_fetched: Option<i64>,
    rates_inserted: Option<i64>,
    rates_closed: Option<i64>,
    error: Option<String>,
    started_at: Option<String>,
    finished_at: Option<String>,
}

impl PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn situation_on(mut self, value: impl Into<String>) -> Self {
        self.situation_on = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: PostV1ReferenceEuVatRatesImportsListResponseRowsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn trigger(
        mut self,
        value: PostV1ReferenceEuVatRatesImportsListResponseRowsItemTrigger,
    ) -> Self {
        self.trigger = Some(value);
        self
    }

    pub fn rates_fetched(mut self, value: i64) -> Self {
        self.rates_fetched = Some(value);
        self
    }

    pub fn rates_inserted(mut self, value: i64) -> Self {
        self.rates_inserted = Some(value);
        self
    }

    pub fn rates_closed(mut self, value: i64) -> Self {
        self.rates_closed = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    pub fn finished_at(mut self, value: impl Into<String>) -> Self {
        self.finished_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesImportsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder::id)
    /// - [`situation_on`](PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder::situation_on)
    /// - [`status`](PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder::status)
    /// - [`trigger`](PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder::trigger)
    /// - [`rates_fetched`](PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder::rates_fetched)
    /// - [`rates_inserted`](PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder::rates_inserted)
    /// - [`rates_closed`](PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder::rates_closed)
    /// - [`started_at`](PostV1ReferenceEuVatRatesImportsListResponseRowsItemBuilder::started_at)
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesImportsListResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceEuVatRatesImportsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            situation_on: self
                .situation_on
                .ok_or_else(|| BuildError::missing_field("situation_on"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            trigger: self
                .trigger
                .ok_or_else(|| BuildError::missing_field("trigger"))?,
            rates_fetched: self
                .rates_fetched
                .ok_or_else(|| BuildError::missing_field("rates_fetched"))?,
            rates_inserted: self
                .rates_inserted
                .ok_or_else(|| BuildError::missing_field("rates_inserted"))?,
            rates_closed: self
                .rates_closed
                .ok_or_else(|| BuildError::missing_field("rates_closed"))?,
            error: self.error,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            finished_at: self.finished_at,
        })
    }
}
