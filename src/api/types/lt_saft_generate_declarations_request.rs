pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtSaftGenerateDeclarationsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<LtSaftGenerateDeclarationsRequestDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,
}

impl LtSaftGenerateDeclarationsRequest {
    pub fn builder() -> LtSaftGenerateDeclarationsRequestBuilder {
        <LtSaftGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSaftGenerateDeclarationsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    data_type: Option<LtSaftGenerateDeclarationsRequestDataType>,
    persist: Option<bool>,
}

impl LtSaftGenerateDeclarationsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn data_type(mut self, value: LtSaftGenerateDeclarationsRequestDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    pub fn persist(mut self, value: bool) -> Self {
        self.persist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtSaftGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](LtSaftGenerateDeclarationsRequestBuilder::from_date)
    /// - [`to_date`](LtSaftGenerateDeclarationsRequestBuilder::to_date)
    pub fn build(self) -> Result<LtSaftGenerateDeclarationsRequest, BuildError> {
        Ok(LtSaftGenerateDeclarationsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            data_type: self.data_type,
            persist: self.persist,
        })
    }
}
