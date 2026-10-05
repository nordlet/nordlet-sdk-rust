pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkV7MGenerateDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "kodUrzedu")]
    #[serde(default)]
    pub kod_urzedu: String,
    #[serde(default)]
    pub email: String,
    #[serde(rename = "celZlozenia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cel_zlozenia: Option<i64>,
}

impl PlJpkV7MGenerateDeclarationsRequest {
    pub fn builder() -> PlJpkV7MGenerateDeclarationsRequestBuilder {
        <PlJpkV7MGenerateDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkV7MGenerateDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    kod_urzedu: Option<String>,
    email: Option<String>,
    cel_zlozenia: Option<i64>,
}

impl PlJpkV7MGenerateDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn kod_urzedu(mut self, value: impl Into<String>) -> Self {
        self.kod_urzedu = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn cel_zlozenia(mut self, value: i64) -> Self {
        self.cel_zlozenia = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlJpkV7MGenerateDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlJpkV7MGenerateDeclarationsRequestBuilder::year)
    /// - [`month`](PlJpkV7MGenerateDeclarationsRequestBuilder::month)
    /// - [`kod_urzedu`](PlJpkV7MGenerateDeclarationsRequestBuilder::kod_urzedu)
    /// - [`email`](PlJpkV7MGenerateDeclarationsRequestBuilder::email)
    pub fn build(self) -> Result<PlJpkV7MGenerateDeclarationsRequest, BuildError> {
        Ok(PlJpkV7MGenerateDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            kod_urzedu: self
                .kod_urzedu
                .ok_or_else(|| BuildError::missing_field("kod_urzedu"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            cel_zlozenia: self.cel_zlozenia,
        })
    }
}
