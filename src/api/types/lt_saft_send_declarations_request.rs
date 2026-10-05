pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtSaftSendDeclarationsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<LtSaftSendDeclarationsRequestDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend: Option<bool>,
}

impl LtSaftSendDeclarationsRequest {
    pub fn builder() -> LtSaftSendDeclarationsRequestBuilder {
        <LtSaftSendDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSaftSendDeclarationsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    data_type: Option<LtSaftSendDeclarationsRequestDataType>,
    confirm: Option<bool>,
    amend: Option<bool>,
}

impl LtSaftSendDeclarationsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn data_type(mut self, value: LtSaftSendDeclarationsRequestDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    pub fn confirm(mut self, value: bool) -> Self {
        self.confirm = Some(value);
        self
    }

    pub fn amend(mut self, value: bool) -> Self {
        self.amend = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtSaftSendDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](LtSaftSendDeclarationsRequestBuilder::from_date)
    /// - [`to_date`](LtSaftSendDeclarationsRequestBuilder::to_date)
    pub fn build(self) -> Result<LtSaftSendDeclarationsRequest, BuildError> {
        Ok(LtSaftSendDeclarationsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            data_type: self.data_type,
            confirm: self.confirm,
            amend: self.amend,
        })
    }
}
