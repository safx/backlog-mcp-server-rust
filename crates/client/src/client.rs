use backlog_api_core::{
    BacklogApiErrorResponse, Error as ApiError, IntoDownloadRequest, IntoRequest,
    IntoUploadRequest, Result, bytes,
};
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use tokio::fs;
use url::Url;

/// A trait for converting HTTP responses into different output types
pub trait IntoResponse {
    type Output;

    /// Convert a reqwest::Response into the desired output type
    #[allow(clippy::wrong_self_convention)]
    fn from_response(
        self,
        response: reqwest::Response,
    ) -> impl std::future::Future<Output = Result<Self::Output>> + Send;
}

/// A marker type to indicate JSON response deserialization
#[derive(Debug)]
pub struct JsonResponse<T>(std::marker::PhantomData<T>);

impl<T> Default for JsonResponse<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T> JsonResponse<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: serde::de::DeserializeOwned + Send,
{
    type Output = T;

    async fn from_response(self, response: reqwest::Response) -> Result<Self::Output> {
        let json_value = response.json::<serde_json::Value>().await?;
        let entity = serde_json::from_value(json_value)?;
        Ok(entity)
    }
}

/// A type that represents a downloaded file's metadata and content.
#[derive(Debug, Clone)]
pub struct DownloadedFile {
    pub filename: String,
    pub content_type: String,
    pub bytes: bytes::Bytes,
}

/// A marker type to indicate file download response
#[derive(Debug)]
pub struct FileResponse;

/// Response handler for 204 No Content responses
#[derive(Debug)]
pub struct NoContentResponse;

impl IntoResponse for NoContentResponse {
    type Output = ();

    async fn from_response(self, response: reqwest::Response) -> Result<Self::Output> {
        match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            status => {
                let error_body = response
                    .text()
                    .await
                    .unwrap_or_else(|e| format!("Failed to read error body: {e}"));
                Err(ApiError::UnexpectedStatus {
                    status: status.as_u16(),
                    body: error_body,
                })
            }
        }
    }
}

impl IntoResponse for FileResponse {
    type Output = DownloadedFile;

    async fn from_response(self, response: reqwest::Response) -> Result<Self::Output> {
        let headers = response.headers().clone();
        let bytes_content = response.bytes().await.map_err(ApiError::from)?;

        // Extract filename from Content-Disposition
        let filename = headers
            .get(CONTENT_DISPOSITION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| {
                // Simple parser for `filename="name.ext"` or `filename*=UTF-8''name.ext`
                if let Some(start) = value.find("filename=\"") {
                    let remainder = &value[start + 10..];
                    remainder.find('"').map(|end| remainder[..end].to_string())
                } else if let Some(start) = value.find("filename*=UTF-8''") {
                    let remainder = &value[start + 17..];
                    // This doesn't handle URL decoding, but it's a start
                    Some(remainder.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "downloaded_file".to_string()); // Default filename

        // Extract Content-Type
        let content_type = headers
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("application/octet-stream") // Default content type
            .to_string();

        Ok(DownloadedFile {
            filename,
            content_type,
            bytes: bytes_content,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
    auth_token: Option<String>,
    api_key: Option<String>,
}

impl Client {
    /// Creates a new Backlog API client
    pub fn new(base_url: &str) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(base_url)?,
            client: reqwest::Client::new(),
            auth_token: None,
            api_key: None,
        })
    }

    /// Sets the authentication token for the client
    pub fn with_auth_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Executes a request using the IntoRequest trait
    pub async fn execute<T, P>(&self, params: P) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Send,
        P: IntoRequest,
    {
        let request = params.into_request(&self.client, &self.base_url)?;
        self.execute_unified(request, JsonResponse::<T>::new())
            .await
    }

    /// Downloads a file using the IntoDownloadRequest trait
    pub async fn download_file<P>(&self, params: P) -> Result<DownloadedFile>
    where
        P: IntoDownloadRequest,
    {
        let request = params.into_request(&self.client, &self.base_url)?;
        self.execute_unified(request, FileResponse).await
    }

    /// Executes a request that returns 204 No Content
    pub async fn execute_no_content<P>(&self, params: P) -> Result<()>
    where
        P: IntoRequest,
    {
        let request = params.into_request(&self.client, &self.base_url)?;
        self.execute_unified(request, NoContentResponse).await
    }

    /// Uploads a file using the IntoUploadRequest trait
    pub async fn upload_file<P, T>(&self, params: P) -> Result<T>
    where
        P: IntoUploadRequest,
        T: serde::de::DeserializeOwned + Send,
    {
        let path = params.path();
        let url = self
            .base_url
            .join(&path)
            .map_err(|e| ApiError::UrlConstruction(format!("Failed to build URL: {e}")))?;
        let file_path = params.file_path().clone();
        let field_name = params.file_field_name().to_string();
        let additional_fields = params.additional_fields();

        // ファイル読み込み
        let file_content = fs::read(&file_path).await.map_err(|e| ApiError::FileRead {
            path: file_path.to_string_lossy().to_string(),
            message: e.to_string(),
        })?;

        let filename = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("attachment")
            .to_string();

        // Multipart form構築
        let file_part = reqwest::multipart::Part::bytes(file_content).file_name(filename);

        let mut form = reqwest::multipart::Form::new().part(field_name.clone(), file_part);

        // 追加フィールドがあれば追加
        for (key, value) in additional_fields {
            form = form.text(key, value);
        }

        let request = self
            .client
            .post(url)
            .multipart(form)
            .build()
            .map_err(|e| ApiError::RequestBuild(format!("Failed to build request: {e}")))?;

        self.execute_unified(request, JsonResponse::<T>::new())
            .await
    }

    /// Unified method for executing requests with customizable response handling
    pub async fn execute_unified<R>(
        &self,
        mut request: reqwest::Request,
        response_handler: R,
    ) -> Result<R::Output>
    where
        R: IntoResponse,
    {
        // Add authentication headers to the request
        if let Some(token) = &self.auth_token {
            let headers = request.headers_mut();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}")
                    .parse()
                    .map_err(|e| ApiError::InvalidAuthToken(format!("Invalid auth token: {e}")))?,
            );
        }

        if let Some(key) = &self.api_key {
            let url = request.url_mut();
            url.query_pairs_mut().append_pair("apiKey", key);
        }

        let response = self.client.execute(request).await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_body_text = response
                .text()
                .await
                .unwrap_or_else(|e| format!("Failed to read error body: {e}"));

            // Attempt to parse as BacklogApiErrorResponse
            match serde_json::from_str::<BacklogApiErrorResponse>(&error_body_text) {
                Ok(parsed_errors) => {
                    let summary = parsed_errors
                        .errors
                        .iter()
                        .map(|e| e.message.clone())
                        .collect::<Vec<String>>()
                        .join("; ");
                    return Err(ApiError::HttpStatus {
                        status,
                        errors: parsed_errors.errors,
                        errors_summary: summary,
                    });
                }
                Err(_) => {
                    return Err(ApiError::UnparseableErrorResponse {
                        status,
                        body: error_body_text,
                    });
                }
            }
        }

        response_handler.from_response(response).await
    }
}
