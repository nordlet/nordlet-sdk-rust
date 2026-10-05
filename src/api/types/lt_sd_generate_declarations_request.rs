pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LtSdGenerateDeclarationsRequest {
    pub r#type: LtSdGenerateDeclarationsRequestType,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
}

impl LtSdGenerateDeclarationsRequest {
    pub fn builder() -> LtSdGenerateDeclarationsRequestBuilder {
        <LtSdGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSdGenerateDeclarationsRequestBuilder {
    r#type: Option<LtSdGenerateDeclarationsRequestType>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
}

impl LtSdGenerateDeclarationsRequestBuilder {
    pub fn r#type(mut self, value: LtSdGenerateDeclarationsRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtSdGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](LtSdGenerateDeclarationsRequestBuilder::r#type)
    /// - [`from_date`](LtSdGenerateDeclarationsRequestBuilder::from_date)
    /// - [`to_date`](LtSdGenerateDeclarationsRequestBuilder::to_date)
    pub fn build(self) -> Result<LtSdGenerateDeclarationsRequest, BuildError> {
        Ok(LtSdGenerateDeclarationsRequest {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
