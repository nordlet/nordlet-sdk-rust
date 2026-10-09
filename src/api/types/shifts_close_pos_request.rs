pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShiftsClosePosRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "countedCash")]
    #[serde(default)]
    pub counted_cash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "reportNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_number: Option<String>,
}

impl ShiftsClosePosRequest {
    pub fn builder() -> ShiftsClosePosRequestBuilder {
        <ShiftsClosePosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShiftsClosePosRequestBuilder {
    id: Option<String>,
    counted_cash: Option<String>,
    date: Option<NaiveDate>,
    report_number: Option<String>,
}

impl ShiftsClosePosRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn counted_cash(mut self, value: impl Into<String>) -> Self {
        self.counted_cash = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn report_number(mut self, value: impl Into<String>) -> Self {
        self.report_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShiftsClosePosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ShiftsClosePosRequestBuilder::id)
    /// - [`counted_cash`](ShiftsClosePosRequestBuilder::counted_cash)
    pub fn build(self) -> Result<ShiftsClosePosRequest, BuildError> {
        Ok(ShiftsClosePosRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            counted_cash: self
                .counted_cash
                .ok_or_else(|| BuildError::missing_field("counted_cash"))?,
            date: self.date,
            report_number: self.report_number,
        })
    }
}
