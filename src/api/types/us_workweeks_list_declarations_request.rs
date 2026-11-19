pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsWorkweeksListDeclarationsRequest {
    #[serde(default)]
    pub from: NaiveDate,
    #[serde(default)]
    pub to: NaiveDate,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
}

impl UsWorkweeksListDeclarationsRequest {
    pub fn builder() -> UsWorkweeksListDeclarationsRequestBuilder {
        <UsWorkweeksListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsWorkweeksListDeclarationsRequestBuilder {
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    employee_id: Option<String>,
}

impl UsWorkweeksListDeclarationsRequestBuilder {
    pub fn from(mut self, value: NaiveDate) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: NaiveDate) -> Self {
        self.to = Some(value);
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsWorkweeksListDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](UsWorkweeksListDeclarationsRequestBuilder::from)
    /// - [`to`](UsWorkweeksListDeclarationsRequestBuilder::to)
    pub fn build(self) -> Result<UsWorkweeksListDeclarationsRequest, BuildError> {
        Ok(UsWorkweeksListDeclarationsRequest {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            employee_id: self.employee_id,
        })
    }
}
