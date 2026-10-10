//! API client and types for the Nordlet Accounting API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    AccountClient, AgreementsClient, ApiClient, AssetsClient, AuditClient, BankClient,
    BillingClient, CalendarClient, CaptureClient, CashClient, CatalogClient, ConsolidationClient,
    DeclarationsClient, DocumentSeriesClient, EcommerceClient, FilesClient, FleetClient, HrClient,
    InventoryClient, LeadsClient, LedgerClient, MigrationClient, OfficersClient,
    OperationTypesClient, PartnersClient, PayrollClient, PeppolClient, PlatformSellersClient,
    PosClient, ProductionClient, ProjectsClient, PublicClient, PurchasesClient, ReferenceClient,
    ReportsClient, SalesClient, TransportClient, WebhooksClient,
};
pub use types::*;
