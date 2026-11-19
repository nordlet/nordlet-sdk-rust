pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Us941GetDeclarationsResponseScheduleBItem {
    #[serde(rename = "payDate")]
    #[serde(default)]
    pub pay_date: NaiveDate,
    #[serde(default)]
    pub liability: String,
}

impl Us941GetDeclarationsResponseScheduleBItem {
    pub fn builder() -> Us941GetDeclarationsResponseScheduleBItemBuilder {
        <Us941GetDeclarationsResponseScheduleBItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us941GetDeclarationsResponseScheduleBItemBuilder {
    pay_date: Option<NaiveDate>,
    liability: Option<String>,
}

impl Us941GetDeclarationsResponseScheduleBItemBuilder {
    pub fn pay_date(mut self, value: NaiveDate) -> Self {
        self.pay_date = Some(value);
        self
    }

    pub fn liability(mut self, value: impl Into<String>) -> Self {
        self.liability = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Us941GetDeclarationsResponseScheduleBItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pay_date`](Us941GetDeclarationsResponseScheduleBItemBuilder::pay_date)
    /// - [`liability`](Us941GetDeclarationsResponseScheduleBItemBuilder::liability)
    pub fn build(self) -> Result<Us941GetDeclarationsResponseScheduleBItem, BuildError> {
        Ok(Us941GetDeclarationsResponseScheduleBItem {
            pay_date: self
                .pay_date
                .ok_or_else(|| BuildError::missing_field("pay_date"))?,
            liability: self
                .liability
                .ok_or_else(|| BuildError::missing_field("liability"))?,
        })
    }
}
