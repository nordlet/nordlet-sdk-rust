pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIsafGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<LtIsafGenerateDeclarationsRequestDataType>,
}

impl LtIsafGenerateDeclarationsRequest {
    pub fn builder() -> LtIsafGenerateDeclarationsRequestBuilder {
        <LtIsafGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIsafGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    data_type: Option<LtIsafGenerateDeclarationsRequestDataType>,
}

impl LtIsafGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn data_type(mut self, value: LtIsafGenerateDeclarationsRequestDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIsafGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtIsafGenerateDeclarationsRequestBuilder::year)
    /// - [`month`](LtIsafGenerateDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<LtIsafGenerateDeclarationsRequest, BuildError> {
        Ok(LtIsafGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            data_type: self.data_type,
        })
    }
}
