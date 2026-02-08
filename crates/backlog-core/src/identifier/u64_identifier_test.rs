//! Tests for u64-based identifier types (SvnRevision, PullRequestNumber).
//! These tests verify that u64 identifiers can handle values larger than u32::MAX.

#[cfg(test)]
mod svn_revision_tests {
    use crate::identifier::{Identifier, SvnRevision};

    #[test]
    fn test_svn_revision_new() {
        let id = SvnRevision::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_svn_revision_large_value() {
        // Test value larger than u32::MAX (4,294,967,295)
        let large_value: u64 = u32::MAX as u64 + 1;
        let id = SvnRevision::new(large_value);
        assert_eq!(id.value(), large_value);
    }

    #[test]
    fn test_svn_revision_display() {
        let id = SvnRevision::new(99);
        assert_eq!(id.to_string(), "99");
        assert_eq!(format!("{}", id), "99");
    }

    #[test]
    fn test_svn_revision_from_str() {
        let result = "123".parse::<SvnRevision>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 123);
    }

    #[test]
    fn test_svn_revision_from_str_large_value() {
        // Test parsing value larger than u32::MAX
        let large_str = "5000000000"; // > u32::MAX
        let result = large_str.parse::<SvnRevision>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 5_000_000_000);
    }

    #[test]
    fn test_svn_revision_from_str_invalid() {
        assert!("abc".parse::<SvnRevision>().is_err());
        assert!("".parse::<SvnRevision>().is_err());
        assert!("-1".parse::<SvnRevision>().is_err());
    }

    #[test]
    fn test_svn_revision_serialize() {
        let id = SvnRevision::new(456);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "456");
    }

    #[test]
    fn test_svn_revision_deserialize() {
        let id: SvnRevision = serde_json::from_str("789").unwrap();
        assert_eq!(id.value(), 789);
    }

    #[test]
    fn test_svn_revision_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SvnRevision::new(1));
        set.insert(SvnRevision::new(2));
        set.insert(SvnRevision::new(1)); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_pull_request_number_large_value() {
        use crate::identifier::PullRequestNumber;
        // Test value larger than u32::MAX
        let large_value: u64 = u32::MAX as u64 + 100;
        let id = PullRequestNumber::new(large_value);
        assert_eq!(id.value(), large_value);
    }
}
