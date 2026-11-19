pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Us940GetDeclarationsResponse {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub year: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1a: Option<String>,
    #[serde(default)]
    pub line1b: bool,
    #[serde(default)]
    pub line3: String,
    #[serde(default)]
    pub line4: String,
    #[serde(default)]
    pub line5: String,
    #[serde(default)]
    pub line6: String,
    #[serde(default)]
    pub line7: String,
    #[serde(default)]
    pub line8: String,
    #[serde(default)]
    pub line12: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part5: Option<Us940GetDeclarationsResponsePart5>,
}

impl Us940GetDeclarationsResponse {
    pub fn builder() -> Us940GetDeclarationsResponseBuilder {
        <Us940GetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us940GetDeclarationsResponseBuilder {
    year: Option<f64>,
    line1a: Option<String>,
    line1b: Option<bool>,
    line3: Option<String>,
    line4: Option<String>,
    line5: Option<String>,
    line6: Option<String>,
    line7: Option<String>,
    line8: Option<String>,
    line12: Option<String>,
    part5: Option<Us940GetDeclarationsResponsePart5>,
}

impl Us940GetDeclarationsResponseBuilder {
    pub fn year(mut self, value: f64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn line1a(mut self, value: impl Into<String>) -> Self {
        self.line1a = Some(value.into());
        self
    }

    pub fn line1b(mut self, value: bool) -> Self {
        self.line1b = Some(value);
        self
    }

    pub fn line3(mut self, value: impl Into<String>) -> Self {
        self.line3 = Some(value.into());
        self
    }

    pub fn line4(mut self, value: impl Into<String>) -> Self {
        self.line4 = Some(value.into());
        self
    }

    pub fn line5(mut self, value: impl Into<String>) -> Self {
        self.line5 = Some(value.into());
        self
    }

    pub fn line6(mut self, value: impl Into<String>) -> Self {
        self.line6 = Some(value.into());
        self
    }

    pub fn line7(mut self, value: impl Into<String>) -> Self {
        self.line7 = Some(value.into());
        self
    }

    pub fn line8(mut self, value: impl Into<String>) -> Self {
        self.line8 = Some(value.into());
        self
    }

    pub fn line12(mut self, value: impl Into<String>) -> Self {
        self.line12 = Some(value.into());
        self
    }

    pub fn part5(mut self, value: Us940GetDeclarationsResponsePart5) -> Self {
        self.part5 = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Us940GetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](Us940GetDeclarationsResponseBuilder::year)
    /// - [`line1b`](Us940GetDeclarationsResponseBuilder::line1b)
    /// - [`line3`](Us940GetDeclarationsResponseBuilder::line3)
    /// - [`line4`](Us940GetDeclarationsResponseBuilder::line4)
    /// - [`line5`](Us940GetDeclarationsResponseBuilder::line5)
    /// - [`line6`](Us940GetDeclarationsResponseBuilder::line6)
    /// - [`line7`](Us940GetDeclarationsResponseBuilder::line7)
    /// - [`line8`](Us940GetDeclarationsResponseBuilder::line8)
    /// - [`line12`](Us940GetDeclarationsResponseBuilder::line12)
    pub fn build(self) -> Result<Us940GetDeclarationsResponse, BuildError> {
        Ok(Us940GetDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            line1a: self.line1a,
            line1b: self
                .line1b
                .ok_or_else(|| BuildError::missing_field("line1b"))?,
            line3: self
                .line3
                .ok_or_else(|| BuildError::missing_field("line3"))?,
            line4: self
                .line4
                .ok_or_else(|| BuildError::missing_field("line4"))?,
            line5: self
                .line5
                .ok_or_else(|| BuildError::missing_field("line5"))?,
            line6: self
                .line6
                .ok_or_else(|| BuildError::missing_field("line6"))?,
            line7: self
                .line7
                .ok_or_else(|| BuildError::missing_field("line7"))?,
            line8: self
                .line8
                .ok_or_else(|| BuildError::missing_field("line8"))?,
            line12: self
                .line12
                .ok_or_else(|| BuildError::missing_field("line12"))?,
            part5: self.part5,
        })
    }
}
