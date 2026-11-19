pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsStateSalesGetDeclarationsResponse {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub states: Vec<UsStateSalesGetDeclarationsResponseStatesItem>,
    #[serde(rename = "withoutState")]
    #[serde(default)]
    pub without_state: UsStateSalesGetDeclarationsResponseWithoutState,
    #[serde(rename = "outsideUs")]
    #[serde(default)]
    pub outside_us: UsStateSalesGetDeclarationsResponseOutsideUs,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl UsStateSalesGetDeclarationsResponse {
    pub fn builder() -> UsStateSalesGetDeclarationsResponseBuilder {
        <UsStateSalesGetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsStateSalesGetDeclarationsResponseBuilder {
    from: Option<String>,
    to: Option<String>,
    states: Option<Vec<UsStateSalesGetDeclarationsResponseStatesItem>>,
    without_state: Option<UsStateSalesGetDeclarationsResponseWithoutState>,
    outside_us: Option<UsStateSalesGetDeclarationsResponseOutsideUs>,
    warnings: Option<Vec<String>>,
}

impl UsStateSalesGetDeclarationsResponseBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn states(mut self, value: Vec<UsStateSalesGetDeclarationsResponseStatesItem>) -> Self {
        self.states = Some(value);
        self
    }

    pub fn without_state(mut self, value: UsStateSalesGetDeclarationsResponseWithoutState) -> Self {
        self.without_state = Some(value);
        self
    }

    pub fn outside_us(mut self, value: UsStateSalesGetDeclarationsResponseOutsideUs) -> Self {
        self.outside_us = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsStateSalesGetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](UsStateSalesGetDeclarationsResponseBuilder::from)
    /// - [`to`](UsStateSalesGetDeclarationsResponseBuilder::to)
    /// - [`states`](UsStateSalesGetDeclarationsResponseBuilder::states)
    /// - [`without_state`](UsStateSalesGetDeclarationsResponseBuilder::without_state)
    /// - [`outside_us`](UsStateSalesGetDeclarationsResponseBuilder::outside_us)
    /// - [`warnings`](UsStateSalesGetDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<UsStateSalesGetDeclarationsResponse, BuildError> {
        Ok(UsStateSalesGetDeclarationsResponse {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            states: self
                .states
                .ok_or_else(|| BuildError::missing_field("states"))?,
            without_state: self
                .without_state
                .ok_or_else(|| BuildError::missing_field("without_state"))?,
            outside_us: self
                .outside_us
                .ok_or_else(|| BuildError::missing_field("outside_us"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
