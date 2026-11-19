pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsW2ListDeclarationsResponse {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub year: f64,
    #[serde(default)]
    pub forms: Vec<UsW2ListDeclarationsResponseFormsItem>,
    #[serde(default)]
    pub w3: UsW2ListDeclarationsResponseW3,
}

impl UsW2ListDeclarationsResponse {
    pub fn builder() -> UsW2ListDeclarationsResponseBuilder {
        <UsW2ListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsW2ListDeclarationsResponseBuilder {
    year: Option<f64>,
    forms: Option<Vec<UsW2ListDeclarationsResponseFormsItem>>,
    w3: Option<UsW2ListDeclarationsResponseW3>,
}

impl UsW2ListDeclarationsResponseBuilder {
    pub fn year(mut self, value: f64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn forms(mut self, value: Vec<UsW2ListDeclarationsResponseFormsItem>) -> Self {
        self.forms = Some(value);
        self
    }

    pub fn w3(mut self, value: UsW2ListDeclarationsResponseW3) -> Self {
        self.w3 = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsW2ListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](UsW2ListDeclarationsResponseBuilder::year)
    /// - [`forms`](UsW2ListDeclarationsResponseBuilder::forms)
    /// - [`w3`](UsW2ListDeclarationsResponseBuilder::w3)
    pub fn build(self) -> Result<UsW2ListDeclarationsResponse, BuildError> {
        Ok(UsW2ListDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            forms: self
                .forms
                .ok_or_else(|| BuildError::missing_field("forms"))?,
            w3: self.w3.ok_or_else(|| BuildError::missing_field("w3"))?,
        })
    }
}
