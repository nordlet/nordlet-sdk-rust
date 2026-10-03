pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsAnnualAccountsAttachmentsAddResponseKind {
    FullReport,
    Notes,
    ManagementReport,
    AuditorStatement,
    AppropriationResolution,
    ApprovalCertificate,
    GeneralDataSheet,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsAnnualAccountsAttachmentsAddResponseKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FullReport => serializer.serialize_str("full_report"),
            Self::Notes => serializer.serialize_str("notes"),
            Self::ManagementReport => serializer.serialize_str("management_report"),
            Self::AuditorStatement => serializer.serialize_str("auditor_statement"),
            Self::AppropriationResolution => serializer.serialize_str("appropriation_resolution"),
            Self::ApprovalCertificate => serializer.serialize_str("approval_certificate"),
            Self::GeneralDataSheet => serializer.serialize_str("general_data_sheet"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsAnnualAccountsAttachmentsAddResponseKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "full_report" => Ok(Self::FullReport),
            "notes" => Ok(Self::Notes),
            "management_report" => Ok(Self::ManagementReport),
            "auditor_statement" => Ok(Self::AuditorStatement),
            "appropriation_resolution" => Ok(Self::AppropriationResolution),
            "approval_certificate" => Ok(Self::ApprovalCertificate),
            "general_data_sheet" => Ok(Self::GeneralDataSheet),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsAnnualAccountsAttachmentsAddResponseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FullReport => write!(f, "full_report"),
            Self::Notes => write!(f, "notes"),
            Self::ManagementReport => write!(f, "management_report"),
            Self::AuditorStatement => write!(f, "auditor_statement"),
            Self::AppropriationResolution => write!(f, "appropriation_resolution"),
            Self::ApprovalCertificate => write!(f, "approval_certificate"),
            Self::GeneralDataSheet => write!(f, "general_data_sheet"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
