use crate::error::Error;
use crate::project_cache::{CacheConfig, ProjectCacheManager};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::ProjectId;
use backlog_core::{ProjectIdOrKey, ProjectKey};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AccessControl {
    allowed_projects: Option<Vec<ProjectKey>>,
    project_cache: Arc<ProjectCacheManager>,
}

impl AccessControl {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let env_value = env::var("BACKLOG_PROJECTS").ok();

        let allowed_projects = if let Some(value) = env_value {
            if value.trim().is_empty() {
                None
            } else {
                let keys: Vec<ProjectKey> = value
                    .split(',')
                    .map(|p| p.trim())
                    .filter(|p| !p.is_empty())
                    .map(ProjectKey::from_str)
                    .collect::<Result<_, _>>()?;

                if keys.is_empty() { None } else { Some(keys) }
            }
        } else {
            None
        };

        // Cache configuration: 5 minute TTL, max 1000 projects
        let cache_config = CacheConfig {
            ttl: Some(Duration::from_secs(300)),
            max_size: Some(1000),
        };
        let project_cache = Arc::new(ProjectCacheManager::with_config(cache_config));

        Ok(Self {
            allowed_projects,
            project_cache,
        })
    }

    async fn resolve_project_by_id(
        &self,
        project_id: &ProjectId,
        client: &BacklogApiClient,
    ) -> Result<ProjectKey, Error> {
        let project = self
            .project_cache
            .get_by_id(project_id, client)
            .await
            .map_err(|e| {
                Error::Parameter(format!("Failed to resolve project ID '{project_id}': {e}"))
            })?;

        Ok(project.project_key.clone())
    }

    pub async fn check_project_access_by_id_async(
        &self,
        project_id: &ProjectId,
        client: &BacklogApiClient,
    ) -> Result<(), Error> {
        if !self.is_enabled() {
            return Ok(());
        }
        let allowed_keys = self.allowed_projects.as_ref().unwrap();

        if let Some(project) = self.project_cache.get_from_cache_by_id(project_id).await {
            if allowed_keys.contains(&project.project_key) {
                return Ok(());
            }
        }

        if let Ok(project_key) = self.resolve_project_by_id(project_id, client).await {
            if allowed_keys.contains(&project_key) {
                return Ok(());
            }
        }

        Err(Error::ProjectAccessDenied {
            project: project_id.to_string(),
            allowed_projects: allowed_keys.iter().map(|k| k.to_string()).collect(),
        })
    }

    pub async fn check_project_access_by_key_async(
        &self,
        project_key: &ProjectKey,
    ) -> Result<(), Error> {
        if !self.is_enabled() {
            return Ok(());
        }

        let allowed_keys = self.allowed_projects.as_ref().unwrap();
        if allowed_keys.contains(project_key) {
            return Ok(());
        }

        Err(Error::ProjectAccessDenied {
            project: project_key.to_string(),
            allowed_projects: allowed_keys.iter().map(|k| k.to_string()).collect(),
        })
    }

    pub async fn check_project_access_id_or_key_async(
        &self,
        project: &ProjectIdOrKey,
        client: &BacklogApiClient,
    ) -> Result<(), Error> {
        match project {
            ProjectIdOrKey::Id(id) => self.check_project_access_by_id_async(id, client).await,
            ProjectIdOrKey::Key(key) => self.check_project_access_by_key_async(key).await,
            ProjectIdOrKey::EitherIdOrKey(id, _) => {
                self.check_project_access_by_id_async(id, client).await
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.allowed_projects.is_some()
    }

    pub fn project_cache(&self) -> &Arc<ProjectCacheManager> {
        &self.project_cache
    }

    // Synchronous versions for backward compatibility (will be removed)

    pub fn check_project_access_by_id(&self, project_id: &ProjectId) -> Result<(), Error> {
        if !self.is_enabled() {
            return Ok(());
        }

        let allowed_keys = self.allowed_projects.as_ref().unwrap();
        Err(Error::ProjectAccessDenied {
            project: project_id.to_string(),
            allowed_projects: allowed_keys.iter().map(|k| k.to_string()).collect(),
        })
    }

    pub fn check_project_access_by_key(&self, project_key: &ProjectKey) -> Result<(), Error> {
        if !self.is_enabled() {
            return Ok(());
        }

        if let Some(allowed_keys) = &self.allowed_projects {
            if allowed_keys.contains(project_key) {
                return Ok(());
            }
        }

        Err(Error::ProjectAccessDenied {
            project: project_key.to_string(),
            allowed_projects: self
                .allowed_projects
                .as_ref()
                .unwrap()
                .iter()
                .map(|key| key.to_string())
                .collect::<Vec<String>>(),
        })
    }

    pub fn check_project_access_id_or_key(&self, project: &ProjectIdOrKey) -> Result<(), Error> {
        match project {
            ProjectIdOrKey::Id(id) => self.check_project_access_by_id(id),
            ProjectIdOrKey::Key(key) => self.check_project_access_by_key(key),
            ProjectIdOrKey::EitherIdOrKey(id, _) => self.check_project_access_by_id(id),
        }
    }
}

impl Default for AccessControl {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            let cache_config = CacheConfig {
                ttl: Some(Duration::from_secs(300)),
                max_size: Some(1000),
            };
            Self {
                allowed_projects: None,
                project_cache: Arc::new(ProjectCacheManager::with_config(cache_config)),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_access_control_disabled_by_default() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("BACKLOG_PROJECTS");
        }
        let access_control = AccessControl::new().unwrap();
        assert!(!access_control.is_enabled());
        let project_key = ProjectKey::from_str("ANY_PROJECT").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_key)
                .is_ok()
        );
    }

    #[test]
    fn test_access_control_with_project_keys() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B,PROJECT_C");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());

        let project_a = ProjectKey::from_str("PROJECT_A").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_a)
                .is_ok()
        );

        let project_b = ProjectKey::from_str("PROJECT_B").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_b)
                .is_ok()
        );

        let project_c = ProjectKey::from_str("PROJECT_C").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_c)
                .is_ok()
        );

        let project_d = ProjectKey::from_str("PROJECT_D").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_d)
                .is_err()
        );
    }

    #[test]
    fn test_access_control_mixed_keys_and_ids() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A, PROJECT_C");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());

        let project_a = ProjectKey::from_str("PROJECT_A").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_a)
                .is_ok()
        );

        let project_c = ProjectKey::from_str("PROJECT_C").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_c)
                .is_ok()
        );

        let project_b = ProjectKey::from_str("PROJECT_B").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_b)
                .is_err()
        );
    }

    #[test]
    fn test_access_control_empty_string() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "");
        }
        let access_control = AccessControl::new().unwrap();
        assert!(!access_control.is_enabled());
    }

    #[test]
    fn test_access_control_whitespace_handling() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", " PROJECT_A , PROJECT_B , ");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());

        let project_a = ProjectKey::from_str("PROJECT_A").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_a)
                .is_ok()
        );

        let project_b = ProjectKey::from_str("PROJECT_B").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_b)
                .is_ok()
        );
    }

    #[test]
    fn test_access_control_phase2_document_api() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_X");
        }
        let access_control = AccessControl::new().unwrap();

        let project_x = ProjectKey::from_str("PROJECT_X").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_x)
                .is_ok()
        );

        let project_id_err = ProjectId::new(999999);
        assert!(
            access_control
                .check_project_access_by_id(&project_id_err)
                .is_err()
        );

        let project_y = ProjectKey::from_str("PROJECT_Y").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_y)
                .is_err()
        );
    }

    #[test]
    fn test_access_control_phase2_wiki_api() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "WIKI_PROJ");
        }
        let access_control = AccessControl::new().unwrap();

        let wiki_proj = ProjectKey::from_str("WIKI_PROJ").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&wiki_proj)
                .is_ok()
        );

        let other_proj = ProjectKey::from_str("OTHER_PROJ").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&other_proj)
                .is_err()
        );

        let err = access_control
            .check_project_access_by_key(&other_proj)
            .unwrap_err();
        match err {
            crate::error::Error::ProjectAccessDenied {
                project,
                allowed_projects,
            } => {
                assert_eq!(project, "OTHER_PROJ");
                assert_eq!(allowed_projects, vec!["WIKI_PROJ"]);
            }
            _ => panic!("Expected ProjectAccessDenied error"),
        }
    }

    #[test]
    fn test_check_project_access_by_id_when_disabled() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("BACKLOG_PROJECTS");
        }
        let access_control = AccessControl::new().unwrap();

        let project_id = ProjectId::new(123456);
        assert!(
            access_control
                .check_project_access_by_id(&project_id)
                .is_ok()
        );
    }

    #[test]
    fn test_check_project_access_by_key_with_allowed_key() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B");
        }
        let access_control = AccessControl::new().unwrap();

        let project_key = ProjectKey::from_str("PROJECT_A").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_key)
                .is_ok()
        );

        let project_key = ProjectKey::from_str("PROJECT_B").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_key)
                .is_ok()
        );
    }

    #[test]
    fn test_check_project_access_by_key_with_denied_key() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B");
        }
        let access_control = AccessControl::new().unwrap();

        let project_key = ProjectKey::from_str("PROJECT_C").unwrap();
        let err = access_control
            .check_project_access_by_key(&project_key)
            .unwrap_err();
        match err {
            crate::error::Error::ProjectAccessDenied {
                project,
                allowed_projects,
            } => {
                assert_eq!(project, "PROJECT_C");
                assert_eq!(allowed_projects, vec!["PROJECT_A", "PROJECT_B"]);
            }
            _ => panic!("Expected ProjectAccessDenied error"),
        }
    }

    #[test]
    fn test_check_project_access_by_key_when_disabled() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("BACKLOG_PROJECTS");
        }
        let access_control = AccessControl::new().unwrap();

        let project_key = ProjectKey::from_str("ANY_PROJECT").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_key)
                .is_ok()
        );
    }
}
