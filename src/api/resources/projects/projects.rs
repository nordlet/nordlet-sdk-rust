use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ProjectsClient {
    pub http_client: HttpClient,
}

impl ProjectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a project; `partnerId` is the customer that `projects/time-entries/bill` invoices by default. A duplicate `code` is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a project's name, customer, `status` (`active`, `completed`, `archived`) or notes; only the fields sent are changed. An archived project accepts no new time entries.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        request: &UpdateProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one project.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        request: &GetProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List projects. Filterable by `code`, `name`, `status` and `partnerId`; sortable by `code`, `name` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ListProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record hours worked on a project on a date, optionally for an employee and with the `hourlyRate` used when billing. `hours` must be positive (422); an archived project is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn time_entries_create(
        &self,
        request: &TimeEntriesCreateProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesCreateProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a time entry; only the fields sent are changed. `hours` must be positive (422); an entry already billed to an invoice cannot be changed (409).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn time_entries_update(
        &self,
        request: &TimeEntriesUpdateProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesUpdateProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a time entry (409 once it is billed).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn time_entries_delete(
        &self,
        request: &TimeEntriesDeleteProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesDeleteProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List time entries. Filterable by `projectId`, `employeeId`, `billable`, `billedInvoiceId` and `date`; sortable by `date`, `createdAt` and `hours`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn time_entries_list(
        &self,
        request: &TimeEntriesListProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesListProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a draft sale invoice from the project's unbilled billable time entries, optionally limited to `dateFrom` and `dateTo`, and mark them billed; needs the `sales:write` scope, and the invoice is issued with `sales/invoices/issue`. The customer is `partnerId`, else the project's partner (422 without either); each entry is priced at its own `hourlyRate`, else the call's `hourlyRate` (422 when an entry has neither); `groupBy` `rate` makes one line per rate with the summed hours, `entry` one line per entry; `itemId`, `vatRatePercent` and `vatClassifierCode` go onto every line. No matching entries is 422.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn time_entries_bill(
        &self,
        request: &TimeEntriesBillProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesBillProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/bill",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Profitability per project: `revenue` and `costs` from posted journal entries tagged with the project (income accounts credit less debit, expense accounts debit less credit), `profit` as their difference, and the hours from time entries (total, billable, billed, unbilled, and the unbilled amount at the entries' hourly rates). `projectId` limits the report to one project; `dateFrom` and `dateTo` limit the journal dates and the entry dates. Nothing is changed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn report(
        &self,
        request: &ReportProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportProjectsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/report",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
