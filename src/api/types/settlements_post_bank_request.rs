pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettlementsPostBankRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "commissionPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_percent: Option<String>,
}

impl SettlementsPostBankRequest {
    pub fn builder() -> SettlementsPostBankRequestBuilder {
        <SettlementsPostBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsPostBankRequestBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    commission_percent: Option<String>,
}

impl SettlementsPostBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn commission_percent(mut self, value: impl Into<String>) -> Self {
        self.commission_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SettlementsPostBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SettlementsPostBankRequestBuilder::id)
    pub fn build(self) -> Result<SettlementsPostBankRequest, BuildError> {
        Ok(SettlementsPostBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date,
            commission_percent: self.commission_percent,
        })
    }
}
