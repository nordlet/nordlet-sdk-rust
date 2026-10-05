pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LtSdFfdataDeclarationsRequest {
    pub r#type: LtSdFfdataDeclarationsRequestType,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "managerFullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_full_name: Option<String>,
    #[serde(rename = "preparatorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparator_details: Option<String>,
}

impl LtSdFfdataDeclarationsRequest {
    pub fn builder() -> LtSdFfdataDeclarationsRequestBuilder {
        <LtSdFfdataDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSdFfdataDeclarationsRequestBuilder {
    r#type: Option<LtSdFfdataDeclarationsRequestType>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    manager_full_name: Option<String>,
    preparator_details: Option<String>,
}

impl LtSdFfdataDeclarationsRequestBuilder {
    pub fn r#type(mut self, value: LtSdFfdataDeclarationsRequestType) -> Self {
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

    pub fn manager_full_name(mut self, value: impl Into<String>) -> Self {
        self.manager_full_name = Some(value.into());
        self
    }

    pub fn preparator_details(mut self, value: impl Into<String>) -> Self {
        self.preparator_details = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtSdFfdataDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](LtSdFfdataDeclarationsRequestBuilder::r#type)
    /// - [`from_date`](LtSdFfdataDeclarationsRequestBuilder::from_date)
    /// - [`to_date`](LtSdFfdataDeclarationsRequestBuilder::to_date)
    pub fn build(self) -> Result<LtSdFfdataDeclarationsRequest, BuildError> {
        Ok(LtSdFfdataDeclarationsRequest {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            manager_full_name: self.manager_full_name,
            preparator_details: self.preparator_details,
        })
    }
}
