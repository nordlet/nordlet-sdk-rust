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
    AccountClient, AgreementsClient, ApiClient, AssetsClient, AuditClient, BankClient, CashClient,
    CatalogClient, ConsolidationClient, DeclarationsClient, EcommerceClient, FilesClient, HrClient,
    InventoryClient, LedgerClient, PartnersClient, PayrollClient, PosClient, ProductionClient,
    PublicClient, PurchasesClient, ReferenceClient, ReportsClient, SalesClient, TransportClient,
    WebhooksClient,
};
pub use types::*;
