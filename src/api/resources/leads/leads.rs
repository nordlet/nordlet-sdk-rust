use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LeadsClient {
    pub http_client: HttpClient,
}

impl LeadsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a lead in status `new`, `contacted`, `qualified` or `lost`. `assignedUserId` must be a member of the company and `sourceId` and `typeId` must exist in it; `countryCode` and `currency` are stored in upper case; `documents` attach uploaded files not yet linked to a record, and `notes` become the lead's first notes with the caller as author. Emits `lead.created`.
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
        request: &CreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one lead with its source and type names.
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
        request: &GetLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the given fields of a lead; the clearable fields accept null, `documents` attach more uploaded files, and `countryCode` and `currency` are stored in upper case. A lead that is already converted cannot change `status` (409).
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
        request: &UpdateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a lead with its notes and detaches its files, which stay in the company's files without a record.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        request: &DeleteLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists leads with their source and type names; filterable by `name`, `contactName`, `email`, `phone`, `status`, `sourceId`, `typeId`, `countryCode`, `assignedUserId`, `partnerId`, `estimatedValue` and `createdAt`, sortable by `name`, `status`, `estimatedValue`, `createdAt` and `updatedAt`.
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
        request: &ListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds a note to a lead with the caller as author and refreshes the lead's `updatedAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn notes_create(
        &self,
        request: &NotesCreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<NotesCreateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/notes/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a lead note.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn notes_delete(
        &self,
        request: &NotesDeleteLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<NotesDeleteLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/notes/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the notes of a lead, newest first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn notes_list(
        &self,
        request: &NotesListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<NotesListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/notes/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the files attached to a lead, newest first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn files_list(
        &self,
        request: &FilesListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<FilesListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/files/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a lead source; a `name` already used by a source of the company is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sources_create(
        &self,
        request: &SourcesCreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesCreateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the name or `isActive` flag of a lead source; a name already used by another source of the company is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sources_update(
        &self,
        request: &SourcesUpdateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesUpdateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a lead source; one still referenced by a lead is refused with 409 and should be deactivated with `leads/sources/update` instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sources_delete(
        &self,
        request: &SourcesDeleteLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesDeleteLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists all lead sources of the company, active and inactive, ordered by name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sources_list(
        &self,
        request: &SourcesListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the id and name of the active lead sources, ordered by name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sources_options(
        &self,
        request: &SourcesOptionsLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesOptionsLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/options",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a lead type; a `name` already used by a type of the company is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn types_create(
        &self,
        request: &TypesCreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesCreateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes the name or `isActive` flag of a lead type; a name already used by another type of the company is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn types_update(
        &self,
        request: &TypesUpdateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesUpdateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a lead type; one still referenced by a lead is refused with 409 and should be deactivated with `leads/types/update` instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn types_delete(
        &self,
        request: &TypesDeleteLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesDeleteLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists all lead types of the company, active and inactive, ordered by name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn types_list(
        &self,
        request: &TypesListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the id and name of the active lead types, ordered by name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn types_options(
        &self,
        request: &TypesOptionsLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesOptionsLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/options",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a customer partner from the lead, move the lead files to the partner, copy the lead notes into the partner notes and mark the lead as converted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn convert(
        &self,
        request: &ConvertLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConvertLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/convert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
