pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkV7MGenerateRequest {
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

impl PostV1DeclarationsPlJpkV7MGenerateRequest {
    pub fn builder() -> PostV1DeclarationsPlJpkV7MGenerateRequestBuilder {
        <PostV1DeclarationsPlJpkV7MGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkV7MGenerateRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    kod_urzedu: Option<String>,
    email: Option<String>,
    cel_zlozenia: Option<i64>,
}

impl PostV1DeclarationsPlJpkV7MGenerateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkV7MGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsPlJpkV7MGenerateRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsPlJpkV7MGenerateRequestBuilder::month)
    /// - [`kod_urzedu`](PostV1DeclarationsPlJpkV7MGenerateRequestBuilder::kod_urzedu)
    /// - [`email`](PostV1DeclarationsPlJpkV7MGenerateRequestBuilder::email)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkV7MGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsPlJpkV7MGenerateRequest {
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
