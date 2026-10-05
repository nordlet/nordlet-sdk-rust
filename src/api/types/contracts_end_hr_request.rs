pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContractsEndHrRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: NaiveDate,
    #[serde(rename = "endReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<String>,
}

impl ContractsEndHrRequest {
    pub fn builder() -> ContractsEndHrRequestBuilder {
        <ContractsEndHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContractsEndHrRequestBuilder {
    id: Option<String>,
    end_date: Option<NaiveDate>,
    end_reason: Option<String>,
}

impl ContractsEndHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn end_reason(mut self, value: impl Into<String>) -> Self {
        self.end_reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContractsEndHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ContractsEndHrRequestBuilder::id)
    /// - [`end_date`](ContractsEndHrRequestBuilder::end_date)
    pub fn build(self) -> Result<ContractsEndHrRequest, BuildError> {
        Ok(ContractsEndHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            end_reason: self.end_reason,
        })
    }
}
