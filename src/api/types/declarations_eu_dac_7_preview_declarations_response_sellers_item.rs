pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDac7PreviewDeclarationsResponseSellersItem {
    #[serde(rename = "sellerId")]
    #[serde(default)]
    pub seller_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub reportable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub consideration: String,
    #[serde(default)]
    pub activities: i64,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl EuDac7PreviewDeclarationsResponseSellersItem {
    pub fn builder() -> EuDac7PreviewDeclarationsResponseSellersItemBuilder {
        <EuDac7PreviewDeclarationsResponseSellersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDac7PreviewDeclarationsResponseSellersItemBuilder {
    seller_id: Option<String>,
    name: Option<String>,
    reportable: Option<bool>,
    reason: Option<String>,
    consideration: Option<String>,
    activities: Option<i64>,
    warnings: Option<Vec<String>>,
}

impl EuDac7PreviewDeclarationsResponseSellersItemBuilder {
    pub fn seller_id(mut self, value: impl Into<String>) -> Self {
        self.seller_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn reportable(mut self, value: bool) -> Self {
        self.reportable = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn consideration(mut self, value: impl Into<String>) -> Self {
        self.consideration = Some(value.into());
        self
    }

    pub fn activities(mut self, value: i64) -> Self {
        self.activities = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuDac7PreviewDeclarationsResponseSellersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`seller_id`](EuDac7PreviewDeclarationsResponseSellersItemBuilder::seller_id)
    /// - [`name`](EuDac7PreviewDeclarationsResponseSellersItemBuilder::name)
    /// - [`reportable`](EuDac7PreviewDeclarationsResponseSellersItemBuilder::reportable)
    /// - [`consideration`](EuDac7PreviewDeclarationsResponseSellersItemBuilder::consideration)
    /// - [`activities`](EuDac7PreviewDeclarationsResponseSellersItemBuilder::activities)
    /// - [`warnings`](EuDac7PreviewDeclarationsResponseSellersItemBuilder::warnings)
    pub fn build(self) -> Result<EuDac7PreviewDeclarationsResponseSellersItem, BuildError> {
        Ok(EuDac7PreviewDeclarationsResponseSellersItem {
            seller_id: self
                .seller_id
                .ok_or_else(|| BuildError::missing_field("seller_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            reportable: self
                .reportable
                .ok_or_else(|| BuildError::missing_field("reportable"))?,
            reason: self.reason,
            consideration: self
                .consideration
                .ok_or_else(|| BuildError::missing_field("consideration"))?,
            activities: self
                .activities
                .ok_or_else(|| BuildError::missing_field("activities"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
