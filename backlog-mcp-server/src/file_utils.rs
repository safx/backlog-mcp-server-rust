use backlog_api_client::{DownloadedFile, bytes};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use rmcp::{ErrorData as McpError, model::Content};

#[derive(Debug, Clone)]
pub enum FileFormat {
    Text,
    Image,
    Raw,
}

impl std::str::FromStr for FileFormat {
    type Err = McpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "image" => Ok(FileFormat::Image),
            "text" => Ok(FileFormat::Text),
            "raw" => Ok(FileFormat::Raw),
            other => Err(McpError::invalid_request(
                format!("Invalid format '{other}'. Valid options: 'image', 'text', 'raw'",),
                None,
            )),
        }
    }
}

pub struct FormatDetector;

impl FormatDetector {
    pub fn detect_format(
        downloaded_file: &DownloadedFile,
        explicit_format: Option<FileFormat>,
    ) -> Result<FileFormat, McpError> {
        if let Some(format) = explicit_format {
            Self::validate_format(downloaded_file, &format)?;
            return Ok(format);
        }

        if downloaded_file.content_type.starts_with("image/") {
            Ok(FileFormat::Image)
        } else if Self::is_text_content(&downloaded_file.content_type)
            || Self::is_likely_text(&downloaded_file.bytes)
        {
            Ok(FileFormat::Text)
        } else {
            Ok(FileFormat::Raw)
        }
    }

    fn validate_format(
        downloaded_file: &DownloadedFile,
        format: &FileFormat,
    ) -> Result<(), McpError> {
        match format {
            FileFormat::Image => {
                Self::ensure_image_type(&downloaded_file.content_type, &downloaded_file.filename)
            }
            FileFormat::Text => Self::ensure_text_type(downloaded_file).map(|_| ()),
            FileFormat::Raw => Ok(()),
        }
    }

    fn is_text_content(content_type: &str) -> bool {
        content_type.starts_with("text/")
            || matches!(
                content_type,
                "application/json"
                    | "application/xml"
                    | "application/javascript"
                    | "application/x-httpd-php"
                    | "application/x-sh"
            )
    }

    fn is_likely_text(bytes: &bytes::Bytes) -> bool {
        if bytes.is_empty() {
            return true;
        }

        let sample_size = bytes.len().min(512);
        let sample = &bytes[..sample_size];

        let mut text_chars = 0;
        let mut total_chars = 0;

        for &byte in sample {
            total_chars += 1;
            if byte.is_ascii_graphic()
                || byte.is_ascii_whitespace()
                || (byte > 127 && std::str::from_utf8(&[byte]).is_ok())
            {
                text_chars += 1;
            }
        }

        if total_chars == 0 {
            return true;
        }

        let text_ratio = text_chars as f64 / total_chars as f64;
        text_ratio > 0.85
    }

    fn ensure_image_type(
        content_type: &str,
        filename_for_error_message: &str,
    ) -> Result<(), McpError> {
        if !content_type.starts_with("image/") {
            return Err(McpError::invalid_request(
                format!(
                    "File '{filename_for_error_message}' is not an image. Reported content type: {content_type}"
                ),
                None,
            ));
        }
        Ok(())
    }

    fn ensure_text_type(downloaded_file: &DownloadedFile) -> Result<String, McpError> {
        match String::from_utf8(downloaded_file.bytes.to_vec()) {
            Ok(text_content) => Ok(text_content),
            Err(_) => Err(McpError::invalid_request(
                format!(
                    "File '{}' is not a valid UTF-8 text file.",
                    downloaded_file.filename
                ),
                None,
            )),
        }
    }
}

pub struct SerializableFile {
    filename: String,
    content_type: String,
    content: SerializableFileContent,
}

pub enum SerializableFileContent {
    Image(bytes::Bytes),
    Text(String),
    Raw(bytes::Bytes),
}

impl SerializableFile {
    pub fn new(
        file: DownloadedFile,
        explicit_format: Option<FileFormat>,
    ) -> Result<Self, McpError> {
        let detected_format = FormatDetector::detect_format(&file, explicit_format)?;
        let filename = file.filename.clone();
        let content_type = file.content_type.clone();

        let content = match detected_format {
            FileFormat::Text => {
                let text = FormatDetector::ensure_text_type(&file)?;
                SerializableFileContent::Text(text)
            }
            FileFormat::Image => {
                FormatDetector::ensure_image_type(&file.content_type, &file.filename)?;
                SerializableFileContent::Image(file.bytes)
            }
            FileFormat::Raw => SerializableFileContent::Raw(file.bytes),
        };

        Ok(Self {
            filename,
            content_type,
            content,
        })
    }

    pub fn text(file: DownloadedFile) -> Result<Self, McpError> {
        Self::new(file, Some(FileFormat::Text))
    }

    pub fn image(file: DownloadedFile) -> Result<Self, McpError> {
        Self::new(file, Some(FileFormat::Image))
    }

    pub fn raw(file: DownloadedFile) -> Self {
        Self::new(file, Some(FileFormat::Raw)).unwrap()
    }

    pub fn auto(file: DownloadedFile) -> Result<Self, McpError> {
        Self::new(file, None)
    }
}

impl TryFrom<SerializableFile> for Content {
    type Error = McpError;
    fn try_from(file: SerializableFile) -> Result<Self, Self::Error> {
        match file.content {
            SerializableFileContent::Text(text) => Ok(Content::text(text)),
            SerializableFileContent::Image(bytes) => Ok(Content::image(
                BASE64_STANDARD.encode(bytes),
                file.content_type,
            )),
            SerializableFileContent::Raw(bytes) => Ok(Content::json(serde_json::json!({
                "filename": file.filename,
                "content_type": file.content_type,
                "content": BASE64_STANDARD.encode(bytes),
            }))?),
        }
    }
}
