pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Us941GetDeclarationsResponseLine5C {
    #[serde(default)]
    pub wages: String,
    #[serde(default)]
    pub tax: String,
}

impl Us941GetDeclarationsResponseLine5C {
    pub fn builder() -> Us941GetDeclarationsResponseLine5CBuilder {
        <Us941GetDeclarationsResponseLine5CBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us941GetDeclarationsResponseLine5CBuilder {
    wages: Option<String>,
    tax: Option<String>,
}

impl Us941GetDeclarationsResponseLine5CBuilder {
    pub fn wages(mut self, value: impl Into<String>) -> Self {
        self.wages = Some(value.into());
        self
    }

    pub fn tax(mut self, value: impl Into<String>) -> Self {
        self.tax = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Us941GetDeclarationsResponseLine5C`].
    /// This method will fail if any of the following fields are not set:
    /// - [`wages`](Us941GetDeclarationsResponseLine5CBuilder::wages)
    /// - [`tax`](Us941GetDeclarationsResponseLine5CBuilder::tax)
    pub fn build(self) -> Result<Us941GetDeclarationsResponseLine5C, BuildError> {
        Ok(Us941GetDeclarationsResponseLine5C {
            wages: self
                .wages
                .ok_or_else(|| BuildError::missing_field("wages"))?,
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
        })
    }
}
