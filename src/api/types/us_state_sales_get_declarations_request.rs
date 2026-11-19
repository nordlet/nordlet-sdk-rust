pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsStateSalesGetDeclarationsRequest {
    #[serde(default)]
    pub from: NaiveDate,
    #[serde(default)]
    pub to: NaiveDate,
}

impl UsStateSalesGetDeclarationsRequest {
    pub fn builder() -> UsStateSalesGetDeclarationsRequestBuilder {
        <UsStateSalesGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsStateSalesGetDeclarationsRequestBuilder {
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
}

impl UsStateSalesGetDeclarationsRequestBuilder {
    pub fn from(mut self, value: NaiveDate) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: NaiveDate) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsStateSalesGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](UsStateSalesGetDeclarationsRequestBuilder::from)
    /// - [`to`](UsStateSalesGetDeclarationsRequestBuilder::to)
    pub fn build(self) -> Result<UsStateSalesGetDeclarationsRequest, BuildError> {
        Ok(UsStateSalesGetDeclarationsRequest {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
