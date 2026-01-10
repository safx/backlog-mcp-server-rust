use super::Error;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::convert::TryFrom;
use std::fmt;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use std::str::FromStr;
use std::vec::Vec;

#[repr(i8)]
#[derive(Eq, PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Role {
    Admin = 1,
    User = 2,
    Reporter = 3,
    Viewer = 4,
    Guest = 5, // FIXME: classic plan only
}

impl Role {
    pub fn all() -> Vec<Role> {
        vec![
            Role::Admin,
            Role::User,
            Role::Reporter,
            Role::Viewer,
            Role::Guest,
        ]
    }
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            "reporter" => Ok(Role::Reporter),
            "viewer" => Ok(Role::Viewer),
            "guest" => Ok(Role::Guest),
            _ => Err(Error::InvalidRole(s.to_string())),
        }
    }
}

impl TryFrom<i32> for Role {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Role::Admin as i32 => Ok(Role::Admin),
            x if x == Role::User as i32 => Ok(Role::User),
            x if x == Role::Reporter as i32 => Ok(Role::Reporter),
            x if x == Role::Viewer as i32 => Ok(Role::Viewer),
            x if x == Role::Guest as i32 => Ok(Role::Guest),
            _ => Err(Error::InvalidRoleId(value)),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match *self {
            Role::Admin => write!(f, "admin"),
            Role::User => write!(f, "user"),
            Role::Reporter => write!(f, "reporter"),
            Role::Viewer => write!(f, "viewer"),
            Role::Guest => write!(f, "guest"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_success() {
        assert_eq!(Role::from_str("admin").unwrap(), Role::Admin);
        assert_eq!(Role::from_str("user").unwrap(), Role::User);
        assert_eq!(Role::from_str("reporter").unwrap(), Role::Reporter);
        assert_eq!(Role::from_str("viewer").unwrap(), Role::Viewer);
        assert_eq!(Role::from_str("guest").unwrap(), Role::Guest);
    }

    #[test]
    fn test_from_str_error() {
        assert_eq!(
            Role::from_str("invalid"),
            Err(Error::InvalidRole("invalid".to_string()))
        );
        assert_eq!(
            Role::from_str("Admin"),
            Err(Error::InvalidRole("Admin".to_string()))
        );
    }

    #[test]
    fn test_try_from_i32_success() {
        assert_eq!(Role::try_from(1).unwrap(), Role::Admin);
        assert_eq!(Role::try_from(2).unwrap(), Role::User);
        assert_eq!(Role::try_from(3).unwrap(), Role::Reporter);
        assert_eq!(Role::try_from(4).unwrap(), Role::Viewer);
        assert_eq!(Role::try_from(5).unwrap(), Role::Guest);
    }

    #[test]
    fn test_try_from_i32_error() {
        assert_eq!(Role::try_from(0), Err(Error::InvalidRoleId(0)));
        assert_eq!(Role::try_from(6), Err(Error::InvalidRoleId(6)));
        assert_eq!(Role::try_from(-1), Err(Error::InvalidRoleId(-1)));
    }

    #[test]
    fn test_display() {
        assert_eq!(Role::Admin.to_string(), "admin");
        assert_eq!(Role::User.to_string(), "user");
        assert_eq!(Role::Reporter.to_string(), "reporter");
        assert_eq!(Role::Viewer.to_string(), "viewer");
        assert_eq!(Role::Guest.to_string(), "guest");
    }

    #[test]
    fn test_all() {
        let roles = Role::all();
        assert_eq!(roles.len(), 5);
        assert!(roles.contains(&Role::Admin));
        assert!(roles.contains(&Role::User));
        assert!(roles.contains(&Role::Reporter));
        assert!(roles.contains(&Role::Viewer));
        assert!(roles.contains(&Role::Guest));
    }
}
