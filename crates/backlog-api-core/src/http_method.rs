/// HTTP methods supported by the Backlog API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    /// Convert to reqwest::Method
    /// This is intentionally not a From implementation to keep it internal
    pub(crate) fn to_reqwest(self) -> reqwest::Method {
        match self {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Patch => reqwest::Method::PATCH,
            HttpMethod::Delete => reqwest::Method::DELETE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_reqwest_get() {
        assert_eq!(
            HttpMethod::Get.to_reqwest(),
            reqwest::Method::GET,
            "GET should convert correctly"
        );
    }

    #[test]
    fn test_to_reqwest_post() {
        assert_eq!(
            HttpMethod::Post.to_reqwest(),
            reqwest::Method::POST,
            "POST should convert correctly"
        );
    }

    #[test]
    fn test_to_reqwest_put() {
        assert_eq!(
            HttpMethod::Put.to_reqwest(),
            reqwest::Method::PUT,
            "PUT should convert correctly"
        );
    }

    #[test]
    fn test_to_reqwest_patch() {
        assert_eq!(
            HttpMethod::Patch.to_reqwest(),
            reqwest::Method::PATCH,
            "PATCH should convert correctly"
        );
    }

    #[test]
    fn test_to_reqwest_delete() {
        assert_eq!(
            HttpMethod::Delete.to_reqwest(),
            reqwest::Method::DELETE,
            "DELETE should convert correctly"
        );
    }
}
