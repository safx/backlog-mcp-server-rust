use crate::error::{Error, Result};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::ProjectId;
use backlog_core::{ProjectIdOrKey, ProjectKey};
use backlog_domain_models::Project;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// キャッシュエントリ
#[derive(Debug, Clone)]
struct CacheEntry {
    /// キャッシュされたプロジェクト
    project: Arc<Project>,
    /// キャッシュされた時刻
    cached_at: Instant,
}

impl CacheEntry {
    fn new(project: Arc<Project>) -> Self {
        Self {
            project,
            cached_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: &Option<Duration>) -> bool {
        if let Some(ttl) = ttl {
            self.cached_at.elapsed() > *ttl
        } else {
            false
        }
    }
}

/// キャッシュ設定
#[derive(Debug, Clone, Default)]
pub struct CacheConfig {
    /// キャッシュの有効期限
    pub ttl: Option<Duration>,
    /// 最大キャッシュサイズ
    pub max_size: Option<usize>,
}

/// プロジェクト情報のキャッシュを管理する構造体
#[derive(Debug, Clone)]
pub struct ProjectCacheManager {
    /// ProjectId -> CacheEntry のマッピング
    by_id: Arc<DashMap<ProjectId, CacheEntry>>,
    /// ProjectKey -> CacheEntry のマッピング
    by_key: Arc<DashMap<ProjectKey, CacheEntry>>,
    /// キャッシュ設定
    config: Arc<RwLock<CacheConfig>>,
    /// アクセス順序の追跡（LRU用）
    access_order: Arc<RwLock<Vec<ProjectId>>>,
}

impl ProjectCacheManager {
    /// 新しいキャッシュマネージャーを作成
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// 設定を指定してキャッシュマネージャーを作成
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            by_id: Arc::new(DashMap::new()),
            by_key: Arc::new(DashMap::new()),
            config: Arc::new(RwLock::new(config)),
            access_order: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// プロジェクト情報をキャッシュに保存
    pub async fn cache_project(&self, project: Project) {
        let project_arc = Arc::new(project);
        let entry = CacheEntry::new(project_arc.clone());
        let project_id = project_arc.id;
        let project_key = project_arc.project_key.clone();

        // 容量制限のチェック
        let config = self.config.read().await;
        if let Some(max_size) = config.max_size {
            let current_size = self.by_id.len();
            if current_size >= max_size {
                // LRU: 最も古いエントリを削除
                let mut access_order = self.access_order.write().await;
                if let Some(oldest_id) = access_order.first().cloned() {
                    if let Some((_, entry)) = self.by_id.remove(&oldest_id) {
                        self.by_key.remove(&entry.project.project_key);
                    }
                    access_order.remove(0);
                }
            }
        }
        drop(config);

        // 新しいエントリを追加
        self.by_id.insert(project_id, entry.clone());
        self.by_key.insert(project_key, entry);

        // アクセス順序を更新
        let mut access_order = self.access_order.write().await;
        access_order.retain(|&id| id != project_id);
        access_order.push(project_id);
    }

    /// キャッシュからIDで取得（キャッシュのみ）
    pub async fn get_from_cache_by_id(&self, id: &ProjectId) -> Option<Arc<Project>> {
        let config = self.config.read().await;
        if let Some(entry) = self.by_id.get(id) {
            if !entry.is_expired(&config.ttl) {
                // アクセス順序を更新
                let mut access_order = self.access_order.write().await;
                access_order.retain(|&project_id| project_id != *id);
                access_order.push(*id);
                return Some(entry.project.clone());
            } else {
                // 期限切れのエントリを削除
                drop(entry);
                self.by_id.remove(id);
                if let Some((_, cache_entry)) = self.by_id.remove_if(id, |_, _| true) {
                    self.by_key.remove(&cache_entry.project.project_key);
                }
            }
        }
        None
    }

    /// キャッシュからKeyで取得（キャッシュのみ）
    pub async fn get_from_cache_by_key(&self, key: &ProjectKey) -> Option<Arc<Project>> {
        let config = self.config.read().await;
        if let Some(entry) = self.by_key.get(key) {
            if !entry.is_expired(&config.ttl) {
                // アクセス順序を更新
                let project_id = entry.project.id;
                let mut access_order = self.access_order.write().await;
                access_order.retain(|&id| id != project_id);
                access_order.push(project_id);
                return Some(entry.project.clone());
            } else {
                // 期限切れのエントリを削除
                let project_id = entry.project.id;
                drop(entry);
                self.by_key.remove(key);
                self.by_id.remove(&project_id);
            }
        }
        None
    }

    /// プロジェクトIDからプロジェクト情報を取得（キャッシュまたはAPI）
    pub async fn get_by_id(
        &self,
        id: &ProjectId,
        client: &BacklogApiClient,
    ) -> Result<Arc<Project>> {
        // キャッシュを確認
        if let Some(project) = self.get_from_cache_by_id(id).await {
            return Ok(project);
        }

        // APIから取得
        use backlog_api_client::backlog_project::GetProjectDetailParams;
        let params = GetProjectDetailParams::new(ProjectIdOrKey::Id(*id));
        let project = client.project().get_project(params).await?;

        // キャッシュに保存
        self.cache_project(project.clone()).await;
        Ok(Arc::new(project))
    }

    /// プロジェクトKeyからプロジェクト情報を取得（キャッシュまたはAPI）
    pub async fn get_by_key(
        &self,
        key: &ProjectKey,
        client: &BacklogApiClient,
    ) -> Result<Arc<Project>> {
        // キャッシュを確認
        if let Some(project) = self.get_from_cache_by_key(key).await {
            return Ok(project);
        }

        // APIから取得
        use backlog_api_client::backlog_project::GetProjectDetailParams;
        let params = GetProjectDetailParams::new(ProjectIdOrKey::Key(key.clone()));
        let project = client.project().get_project(params).await?;

        // キャッシュに保存
        self.cache_project(project.clone()).await;
        Ok(Arc::new(project))
    }

    /// ProjectIdOrKeyからプロジェクト情報を解決
    pub async fn resolve(
        &self,
        id_or_key: &ProjectIdOrKey,
        client: &BacklogApiClient,
    ) -> Result<Arc<Project>> {
        match id_or_key {
            ProjectIdOrKey::Id(id) => self.get_by_id(id, client).await,
            ProjectIdOrKey::Key(key) => self.get_by_key(key, client).await,
            ProjectIdOrKey::EitherIdOrKey(id, _) => self.get_by_id(id, client).await,
        }
    }

    /// キャッシュをクリア
    pub async fn clear(&self) {
        self.by_id.clear();
        self.by_key.clear();
        let mut access_order = self.access_order.write().await;
        access_order.clear();
    }

    /// キャッシュのサイズを取得
    pub async fn size(&self) -> usize {
        self.by_id.len()
    }
}

impl Default for ProjectCacheManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_api_client::backlog_project::GetProjectDetailParams;
    use backlog_api_client::client::BacklogApiClient;
    use backlog_core::identifier::{Identifier, ProjectId};
    use backlog_core::{ProjectIdOrKey, ProjectKey};
    use backlog_domain_models::Project;
    use std::str::FromStr;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

    fn create_test_project(id: u32, key: &str) -> Project {
        Project {
            id: ProjectId::new(id),
            project_key: ProjectKey::from_str(key).unwrap(),
            name: format!("Test Project {id}"),
            chart_enabled: false,
            subtasking_enabled: false,
            project_leader_can_edit_project_leader: false,
            use_wiki: false,
            use_file_sharing: false,
            use_wiki_tree_view: false,
            use_original_image_size_at_wiki: false,
            text_formatting_rule: backlog_core::TextFormattingRule::Markdown,
            archived: false,
            display_order: 0,
            use_dev_attributes: false,
        }
    }

    fn create_test_client(base_url: &str) -> BacklogApiClient {
        BacklogApiClient::new(base_url).unwrap()
    }

    #[tokio::test]
    async fn test_cache_and_retrieve_by_id() {
        let cache = ProjectCacheManager::new();
        let project = create_test_project(123, "TEST_PROJ");
        let project_id = project.id;

        // キャッシュに保存
        cache.cache_project(project.clone()).await;

        // IDで取得（キャッシュから）
        let cached = cache.get_from_cache_by_id(&project_id).await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().id, project_id);
    }

    #[tokio::test]
    async fn test_cache_and_retrieve_by_key() {
        let cache = ProjectCacheManager::new();
        let project = create_test_project(456, "ANOTHER_PROJ");
        let project_key = project.project_key.clone();

        // キャッシュに保存
        cache.cache_project(project.clone()).await;

        // Keyで取得（キャッシュから）
        let cached = cache.get_from_cache_by_key(&project_key).await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().project_key, project_key);
    }

    #[tokio::test]
    async fn test_bidirectional_cache() {
        let cache = ProjectCacheManager::new();
        let project = create_test_project(789, "BIDIRECT_PROJ");
        let project_id = project.id;
        let project_key = project.project_key.clone();

        // キャッシュに保存
        cache.cache_project(project.clone()).await;

        // 両方の方法で取得できることを確認
        let by_id = cache.get_from_cache_by_id(&project_id).await;
        let by_key = cache.get_from_cache_by_key(&project_key).await;

        assert!(by_id.is_some());
        assert!(by_key.is_some());
        assert_eq!(by_id.unwrap().id, by_key.unwrap().id);
    }

    #[tokio::test]
    async fn test_get_by_id_with_api_call() {
        let mock_server = MockServer::start().await;
        let cache = ProjectCacheManager::new();
        let client = create_test_client(&mock_server.uri());
        let project_id = ProjectId::new(999);

        // APIモックを設定
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/projects/999"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 999,
                "projectKey": "API_PROJ",
                "name": "API Project",
                "chartEnabled": false,
                "subtaskingEnabled": false,
                "projectLeaderCanEditProjectLeader": false,
                "useWiki": false,
                "useFileSharing": false,
                "useWikiTreeView": false,
                "useOriginalImageSizeAtWiki": false,
                "textFormattingRule": "markdown",
                "archived": false,
                "displayOrder": 0,
                "useDevAttributes": false
            })))
            .mount(&mock_server)
            .await;

        // キャッシュになければAPIから取得
        let result = cache.get_by_id(&project_id, &client).await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.id, project_id);
        assert_eq!(project.project_key.as_str(), "API_PROJ");

        // 再度取得（今度はキャッシュから）
        let cached = cache.get_from_cache_by_id(&project_id).await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().id, project_id);
    }

    #[tokio::test]
    async fn test_get_by_key_with_api_call() {
        let mock_server = MockServer::start().await;
        let cache = ProjectCacheManager::new();
        let client = create_test_client(&mock_server.uri());
        let project_key = ProjectKey::from_str("KEY_PROJ").unwrap();

        // APIモックを設定
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/v2/projects/KEY_PROJ"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1234,
                "projectKey": "KEY_PROJ",
                "name": "Key Project",
                "chartEnabled": false,
                "subtaskingEnabled": false,
                "projectLeaderCanEditProjectLeader": false,
                "useWiki": false,
                "useFileSharing": false,
                "useWikiTreeView": false,
                "useOriginalImageSizeAtWiki": false,
                "textFormattingRule": "markdown",
                "archived": false,
                "displayOrder": 0,
                "useDevAttributes": false
            })))
            .mount(&mock_server)
            .await;

        // キャッシュになければAPIから取得
        let result = cache.get_by_key(&project_key, &client).await;
        assert!(result.is_ok());
        let project = result.unwrap();
        assert_eq!(project.project_key, project_key);
        assert_eq!(project.id.value(), 1234);

        // 再度取得（今度はキャッシュから）
        let cached = cache.get_from_cache_by_key(&project_key).await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().project_key, project_key);
    }

    #[tokio::test]
    async fn test_cache_not_found() {
        let cache = ProjectCacheManager::new();
        let project_id = ProjectId::new(99999);
        let project_key = ProjectKey::from_str("NOTFOUND").unwrap();

        // 存在しないものはNone
        let by_id = cache.get_from_cache_by_id(&project_id).await;
        let by_key = cache.get_from_cache_by_key(&project_key).await;

        assert!(by_id.is_none());
        assert!(by_key.is_none());
    }

    #[tokio::test]
    async fn test_resolve_project_id_or_key() {
        let mock_server = MockServer::start().await;
        let cache = ProjectCacheManager::new();
        let client = create_test_client(&mock_server.uri());

        // テストプロジェクトをキャッシュ
        let project = create_test_project(5555, "RESOLVE_PROJ");
        cache.cache_project(project.clone()).await;

        // IDで解決
        let id_or_key = ProjectIdOrKey::Id(ProjectId::new(5555));
        let result = cache.resolve(&id_or_key, &client).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id.value(), 5555);

        // Keyで解決
        let id_or_key = ProjectIdOrKey::Key(ProjectKey::from_str("RESOLVE_PROJ").unwrap());
        let result = cache.resolve(&id_or_key, &client).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().project_key.as_str(), "RESOLVE_PROJ");

        // EitherIdOrKeyで解決
        let id_or_key = ProjectIdOrKey::EitherIdOrKey(
            ProjectId::new(5555),
            ProjectKey::from_str("RESOLVE_PROJ").unwrap(),
        );
        let result = cache.resolve(&id_or_key, &client).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id.value(), 5555);
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        use std::sync::Arc;
        use tokio::task::JoinSet;

        let cache = Arc::new(ProjectCacheManager::new());
        let mut tasks = JoinSet::new();

        // 複数のタスクから同時にアクセス
        for i in 0..10 {
            let cache_clone = cache.clone();
            tasks.spawn(async move {
                let project = create_test_project(i, &format!("CONCURRENT_{i}"));
                cache_clone.cache_project(project).await;
            });
        }

        // すべてのタスクが完了するのを待つ
        while tasks.join_next().await.is_some() {}

        // すべてのプロジェクトがキャッシュされていることを確認
        for i in 0..10 {
            let project_id = ProjectId::new(i);
            let cached = cache.get_from_cache_by_id(&project_id).await;
            assert!(cached.is_some());
            assert_eq!(cached.unwrap().id.value(), i);
        }
    }

    #[tokio::test]
    async fn test_cache_with_ttl() {
        use std::time::Duration;
        use tokio::time;

        let config = CacheConfig {
            ttl: Some(Duration::from_millis(100)),
            max_size: None,
        };
        let cache = ProjectCacheManager::with_config(config);
        let project = create_test_project(111, "TTL_PROJ");
        let project_id = project.id;

        // キャッシュに保存
        cache.cache_project(project.clone()).await;

        // 直後は取得できる
        let cached = cache.get_from_cache_by_id(&project_id).await;
        assert!(cached.is_some());

        // TTLが過ぎるまで待つ
        time::sleep(Duration::from_millis(150)).await;

        // TTL後は取得できない
        let expired = cache.get_from_cache_by_id(&project_id).await;
        assert!(expired.is_none());
    }

    #[tokio::test]
    async fn test_cache_with_max_size() {
        let config = CacheConfig {
            ttl: None,
            max_size: Some(3),
        };
        let cache = ProjectCacheManager::with_config(config);

        // 容量制限まで追加
        for i in 0..3 {
            let project = create_test_project(i, &format!("SIZE_{i}"));
            cache.cache_project(project).await;
        }

        // すべて取得できることを確認
        for i in 0..3 {
            let project_id = ProjectId::new(i);
            let cached = cache.get_from_cache_by_id(&project_id).await;
            assert!(cached.is_some());
        }

        // 容量を超えて追加
        let project = create_test_project(3, "SIZE_3");
        cache.cache_project(project).await;

        // 最も古いものが削除される（LRU）
        let oldest = cache.get_from_cache_by_id(&ProjectId::new(0)).await;
        assert!(oldest.is_none());

        // 新しいものは存在する
        let newest = cache.get_from_cache_by_id(&ProjectId::new(3)).await;
        assert!(newest.is_some());
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let cache = ProjectCacheManager::new();

        // 複数のプロジェクトを追加
        for i in 0..5 {
            let project = create_test_project(i, &format!("CLEAR_{i}"));
            cache.cache_project(project).await;
        }

        // キャッシュをクリア
        cache.clear().await;

        // すべて削除されていることを確認
        for i in 0..5 {
            let project_id = ProjectId::new(i);
            let cached = cache.get_from_cache_by_id(&project_id).await;
            assert!(cached.is_none());
        }
    }

    #[tokio::test]
    async fn test_cache_size() {
        let cache = ProjectCacheManager::new();

        // サイズ0から開始
        assert_eq!(cache.size().await, 0);

        // プロジェクトを追加
        for i in 0..3 {
            let project = create_test_project(i, &format!("SIZE_TEST_{i}"));
            cache.cache_project(project).await;
        }

        // サイズが正しいことを確認
        assert_eq!(cache.size().await, 3);

        // クリア後はサイズ0
        cache.clear().await;
        assert_eq!(cache.size().await, 0);
    }
}

