pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GbVatObligationsListDeclarationsResponseObligationsItem {
    #[serde(rename = "periodKey")]
    #[serde(default)]
    pub period_key: String,
    #[serde(default)]
    pub start: String,
    #[serde(default)]
    pub end: String,
    #[serde(default)]
    pub due: String,
    pub status: GbVatObligationsListDeclarationsResponseObligationsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<String>,
}

impl GbVatObligationsListDeclarationsResponseObligationsItem {
    pub fn builder() -> GbVatObligationsListDeclarationsResponseObligationsItemBuilder {
        <GbVatObligationsListDeclarationsResponseObligationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatObligationsListDeclarationsResponseObligationsItemBuilder {
    period_key: Option<String>,
    start: Option<String>,
    end: Option<String>,
    due: Option<String>,
    status: Option<GbVatObligationsListDeclarationsResponseObligationsItemStatus>,
    received: Option<String>,
}

impl GbVatObligationsListDeclarationsResponseObligationsItemBuilder {
    pub fn period_key(mut self, value: impl Into<String>) -> Self {
        self.period_key = Some(value.into());
        self
    }

    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }

    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    pub fn due(mut self, value: impl Into<String>) -> Self {
        self.due = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: GbVatObligationsListDeclarationsResponseObligationsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn received(mut self, value: impl Into<String>) -> Self {
        self.received = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbVatObligationsListDeclarationsResponseObligationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period_key`](GbVatObligationsListDeclarationsResponseObligationsItemBuilder::period_key)
    /// - [`start`](GbVatObligationsListDeclarationsResponseObligationsItemBuilder::start)
    /// - [`end`](GbVatObligationsListDeclarationsResponseObligationsItemBuilder::end)
    /// - [`due`](GbVatObligationsListDeclarationsResponseObligationsItemBuilder::due)
    /// - [`status`](GbVatObligationsListDeclarationsResponseObligationsItemBuilder::status)
    pub fn build(
        self,
    ) -> Result<GbVatObligationsListDeclarationsResponseObligationsItem, BuildError> {
        Ok(GbVatObligationsListDeclarationsResponseObligationsItem {
            period_key: self
                .period_key
                .ok_or_else(|| BuildError::missing_field("period_key"))?,
            start: self
                .start
                .ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            due: self.due.ok_or_else(|| BuildError::missing_field("due"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            received: self.received,
        })
    }
}
