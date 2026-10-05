pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubmissionsCreateDeclarationsRequest {
    pub obligation: SubmissionsCreateDeclarationsRequestObligation,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<SubmissionsCreateDeclarationsRequestDataType>,
}

impl SubmissionsCreateDeclarationsRequest {
    pub fn builder() -> SubmissionsCreateDeclarationsRequestBuilder {
        <SubmissionsCreateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsCreateDeclarationsRequestBuilder {
    obligation: Option<SubmissionsCreateDeclarationsRequestObligation>,
    year: Option<i64>,
    month: Option<i64>,
    data_type: Option<SubmissionsCreateDeclarationsRequestDataType>,
}

impl SubmissionsCreateDeclarationsRequestBuilder {
    pub fn obligation(mut self, value: SubmissionsCreateDeclarationsRequestObligation) -> Self {
        self.obligation = Some(value);
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn data_type(mut self, value: SubmissionsCreateDeclarationsRequestDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsCreateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`obligation`](SubmissionsCreateDeclarationsRequestBuilder::obligation)
    /// - [`year`](SubmissionsCreateDeclarationsRequestBuilder::year)
    /// - [`month`](SubmissionsCreateDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<SubmissionsCreateDeclarationsRequest, BuildError> {
        Ok(SubmissionsCreateDeclarationsRequest {
            obligation: self
                .obligation
                .ok_or_else(|| BuildError::missing_field("obligation"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            data_type: self.data_type,
        })
    }
}
